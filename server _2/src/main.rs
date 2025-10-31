mod vendor_bootstrap;
mod cors;

use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use futures::{SinkExt, StreamExt};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::{
    //Path,
    PathBuf,
};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tower_http::services::ServeDir;
use walkdir::WalkDir;

fn spawn_file_watcher(
    ws_path: std::path::PathBuf,
    tx: tokio::sync::broadcast::Sender<String>,
) -> notify::Result<RecommendedWatcher> {
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        if let Ok(ev) = res {
            // pick the first path and publish
            if let Some(p) = ev.paths.get(0) {
                // Only care about modify/write
                match ev.kind {
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                        let _ = tx.send(p.to_string_lossy().into());
                    }
                    _ => {}
                }
            }
        }
    })?;
    watcher.watch(&ws_path, RecursiveMode::Recursive)?;
    Ok(watcher)
}

#[derive(Clone)]
struct AppState {
    workspace: std::path::PathBuf,
    runner_cmd: Option<String>,
    // NEW:
    sandbox: SandboxMode,
    allow_net: bool,
    busybox: Option<std::path::PathBuf>,

    // NEW:
    shell: Shell,
    shell_bin: Option<std::path::PathBuf>,
    tools_dir: Option<std::path::PathBuf>,
    node_bin: Option<std::path::PathBuf>,
    claude_cli: Option<std::path::PathBuf>,
    forward_env: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shell {
    Sh,
    Bash,
    Zsh,
    Tmux,
}

impl Shell {
    fn from_str(s: &str) -> Self {
        match s {
            "bash" => Shell::Bash,
            "zsh" => Shell::Zsh,
            "tmux" => Shell::Tmux,
            _ => Shell::Sh,
        }
    }
}
// Helper function to add environment forwarding and Claude/Node bindings to bwrap command
fn add_env_and_claude_bindings(
    c: &mut portable_pty::CommandBuilder,
    state: &AppState,
    _sink_arc: &SinkHandle,
) -> Option<()> {
    // Forward explicitly requested environment variables
    for k in &state.forward_env {
        if let Ok(v) = std::env::var(k) {
            c.arg("--setenv");
            c.arg(k);
            c.arg(&v);
        }
    }

    // Optionally expose Node + Claude wrapper if configured
    if let (Some(node_bin), Some(claude_js)) = (&state.node_bin, &state.claude_cli) {
        c.arg("--ro-bind");
        c.arg(node_bin.to_string_lossy().as_ref());
        c.arg("/tools/bin/node");
        c.arg("--dir");
        c.arg("/tools/lib/claude");
        c.arg("--ro-bind");
        c.arg(claude_js.to_string_lossy().as_ref());
        c.arg("/tools/lib/claude/cli.js");

        // Bind a tiny wrapper as /tools/bin/claude
        let wrapper_opt = match ensure_claude_wrapper_script() {
            Ok(p) => Some(p),
            Err(_e) => None,
        };
        if let Some(wrapper) = wrapper_opt {
            c.arg("--ro-bind");
            c.arg(wrapper.to_string_lossy().as_ref());
            c.arg("/tools/bin/claude");
        }
    }
    Some(())
}

// Helper function to configure shell command based on state.shell
fn configure_shell_command(c: &mut portable_pty::CommandBuilder, state: &AppState) {
    c.arg("--chdir");
    c.arg("/");

    match state.shell {
        Shell::Sh => {
            // busybox shell (present because you symlinked /bin/sh -> /bin/busybox)
            c.arg("/bin/sh");
            c.arg("-l");
        }
        Shell::Bash => {
            // Bind bash into /tools/bin/bash if user provided a path; otherwise fall back to sh
            if let Some(bin) = &state.shell_bin {
                c.arg("--ro-bind");
                c.arg(bin.to_string_lossy().as_ref());
                c.arg("/tools/bin/bash");
                c.arg("/tools/bin/bash");
                c.arg("-l");
            } else {
                c.arg("/bin/sh");
                c.arg("-l");
            }
        }
        Shell::Zsh => {
            if let Some(bin) = &state.shell_bin {
                c.arg("--ro-bind");
                c.arg(bin.to_string_lossy().as_ref());
                c.arg("/tools/bin/zsh");
                // Optionally bind /usr/share/zsh read-only if you need completion/functions
                if std::path::Path::new("/usr/share/zsh").exists() {
                    c.arg("--ro-bind");
                    c.arg("/usr/share/zsh");
                    c.arg("/usr/share/zsh");
                }
                c.arg("/tools/bin/zsh");
                c.arg("-l");
            } else {
                c.arg("/bin/sh");
                c.arg("-l");
            }
        }
        Shell::Tmux => {
            if let Some(bin) = &state.shell_bin {
                c.arg("--ro-bind");
                c.arg(bin.to_string_lossy().as_ref());
                c.arg("/tools/bin/tmux");
                // terminfo helps tmux; bind if present
                if std::path::Path::new("/usr/share/terminfo").exists() {
                    c.arg("--ro-bind");
                    c.arg("/usr/share/terminfo");
                    c.arg("/usr/share/terminfo");
                }
                c.arg("--setenv");
                c.arg("TERM");
                c.arg("xterm-256color");
                c.arg("/tools/bin/tmux");
            } else {
                // fallback to a shell if tmux not provided
                c.arg("/bin/sh");
                c.arg("-l");
            }
        }
    }
}

fn ensure_claude_wrapper_script() -> Result<std::path::PathBuf, String> {
    use std::io::Write;
    let p = std::env::temp_dir().join("claude-wrapper.sh");

    // If it already exists, just return it.
    match std::fs::metadata(&p) {
        Ok(_) => return Ok(p),
        Err(e) => {
            // proceed to (re)create; ignore NotFound, report other errors only later
            let _ = e;
        }
    }

    // Create file
    let mut file = match std::fs::File::create(&p) {
        Ok(f) => f,
        Err(e) => return Err(format!("create {}: {}", p.display(), e)),
    };

    // Write launcher
    let content = b"#!/bin/sh\nexec /tools/bin/node /tools/lib/claude/cli.js \"$@\"\n";
    match file.write_all(content) {
        Ok(_) => {}
        Err(e) => return Err(format!("write {}: {}", p.display(), e)),
    }

    // chmod +x (unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match std::fs::metadata(&p) {
            Ok(md) => {
                let mut perm = md.permissions();
                perm.set_mode(0o755);
                if let Err(e) = std::fs::set_permissions(&p, perm) {
                    return Err(format!("chmod {}: {}", p.display(), e));
                }
            }
            Err(e) => return Err(format!("stat {}: {}", p.display(), e)),
        }
    }

    Ok(p)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SandboxMode {
    None,
    Bwrap,
}

#[derive(clap::Parser, Debug, Clone)]
#[command(name = "editor-web-daemon", version)]
struct Args {
    #[arg(long)]
    workspace: std::path::PathBuf,
    #[arg(long)]
    runner: Option<String>,
    #[arg(long, default_value_t = 8787)]
    port: u16,
    // NEW:
    /// none | bwrap
    #[arg(long, default_value = "bwrap")]
    sandbox: String,
    /// allow network inside the sandbox
    #[arg(long, default_value_t = false)]
    allow_net: bool,
    /// path to a static busybox binary to use as /bin/sh inside the sandbox
    #[arg(long)]
    busybox: Option<std::path::PathBuf>,
    /// terminal tmux bash zsh
    #[arg(long, default_value = "sh")]
    shell: String, // "sh" | "bash" | "zsh" | "tmux"
    #[arg(long)]
    shell_bin: Option<PathBuf>, // host path to the binary to bind
    #[arg(long)]
    tools_dir: Option<PathBuf>, // host directory with extra tools to bind to /tools/bin
    #[arg(long, action=clap::ArgAction::Append)]
    extra_bind_lib: Vec<PathBuf>, // repeatable; additional libs to bind
    #[arg(long, action=clap::ArgAction::Append)]
    extra_bind_dir: Vec<PathBuf>, // e.g. /usr/share/zsh, /usr/share/terminfo
    /// Claude stuff
    #[arg(long)]
    node_bin: Option<std::path::PathBuf>,
    #[arg(long)]
    claude_cli: Option<std::path::PathBuf>,
    // optional: forward arbitrary envs
    #[arg(long, action = clap::ArgAction::Append)]
    forward_env: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = <Args as clap::Parser>::parse();
    let mode = match args.sandbox.as_str() {
        "bwrap" => SandboxMode::Bwrap,
        _ => SandboxMode::None,
    };
    let state = AppState {
        workspace: args.workspace.clone(),
        runner_cmd: args.runner.clone(),
        sandbox: mode,
        allow_net: args.allow_net,
        busybox: args.busybox.clone(),

        shell: Shell::from_str(&args.shell.to_lowercase()),
        shell_bin: args.shell_bin.clone(),
        tools_dir: args.tools_dir.clone(),
        node_bin: args.node_bin.clone(),
        claude_cli: args.claude_cli.clone(),
        forward_env: args.forward_env.clone(),
    };
    let assets: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    if let Err(e) = vendor_bootstrap::ensure_vendor_assets(&assets).await {
        eprintln!("vendor bootstrap: {}", e);
    }

    // Load CORS configuration
    let cors_config = match cors::CorsConfig::from_file(".env_cors") {
        Ok(config) => {
            println!("CORS config loaded with {} rules", config.configured_origins().len());
            Arc::new(config)
        }
        Err(e) => {
            eprintln!("Warning: Failed to load CORS config: {}", e);
            eprintln!("Creating permissive default CORS config...");
            // Create a default permissive config
            Arc::new(cors::CorsConfig::from_str("* ALLOW ALL").unwrap())
        }
    };

    let static_dir = ServeDir::new(&assets).append_index_html_on_directories(true);

    let app = Router::new()
        .route("/api/ping", get(|| async { "pong" }))
        .route("/api/list", get(list_files))
        .route("/api/open", get(open_file))
        .route("/api/exists", get(file_exists))
        .route("/api/save", post(save_file))
        .route("/api/search", get(search_files))
        .route("/ws", get(ws_handler))
        .with_state(state.clone())
        .layer(axum::middleware::from_fn(move |req, next| {
            let config = cors_config.clone();
            cors::cors_middleware(config, req, next)
        }))
        .fallback_service(static_dir);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    println!("Serving http://{} | workspace {:?}", addr, state.workspace);
    if let Some(r) = &state.runner_cmd {
        println!("Runner available: {}", r);
    }
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct ListQuery {
    #[serde(default)]
    max: Option<usize>,
}
async fn list_files(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> impl IntoResponse {
    let mut out = Vec::<String>::new();
    let max = q.max.unwrap_or(5000);
    for entry in WalkDir::new(&state.workspace)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();
        if p.is_file() {
            if p.components().any(|c| {
                let s = c.as_os_str().to_string_lossy();
                s == "target"
                    || s == "node_modules"
                    || s == ".git"
                    || s == "deps"
                    || s == ".routify"
                    || s == ".yarn"
                    || s == ".vscode"
                    || s == "log"
                    || s == "dist"
            }) {
                continue;
            }
            if let Ok(rel) = p.strip_prefix(&state.workspace) {
                out.push(rel.to_string_lossy().to_string());
                if out.len() >= max {
                    break;
                }
            }
        }
    }
    Json(out)
}

#[derive(Deserialize)]
struct OpenQuery {
    path: String,
}
async fn open_file(State(state): State<AppState>, Query(q): Query<OpenQuery>) -> impl IntoResponse {
    let path = state.workspace.join(&q.path);
    match tokio::fs::read_to_string(&path).await {
        Ok(s) => Json(serde_json::json!({"ok":true, "content": s})),
        Err(e) => Json(serde_json::json!({"ok":false, "error": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct ExistsQuery {
    path: String,
}
async fn file_exists(State(state): State<AppState>, Query(q): Query<ExistsQuery>) -> impl IntoResponse {
    let path = state.workspace.join(&q.path);
    let exists = path.exists() && path.is_file();
    Json(serde_json::json!({"exists": exists}))
}

#[derive(Deserialize)]
struct SaveBody {
    path: String,
    content: String,
}
async fn save_file(State(state): State<AppState>, Json(body): Json<SaveBody>) -> impl IntoResponse {
    let path = state.workspace.join(&body.path);
    if let Some(parent) = path.parent() {
        if let Err(e) = tokio::fs::create_dir_all(parent).await {
            return Json(serde_json::json!({"ok":false, "error": e.to_string()}));
        }
    }
    match tokio::fs::write(&path, body.content.as_bytes()).await {
        Ok(()) => Json(serde_json::json!({"ok":true})),
        Err(e) => Json(serde_json::json!({"ok":false, "error": e.to_string()})),
    }
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    #[serde(default)]
    max: Option<usize>,
}
#[derive(Serialize)]
struct Hit {
    path: String,
    line: usize,
    col: usize,
    text: String,
}
async fn search_files(
    State(state): State<AppState>,
    Query(q): Query<SearchQuery>,
) -> impl IntoResponse {
    let max = q.max.unwrap_or(200);
    let mut results = Vec::<Hit>::new();
    for entry in WalkDir::new(&state.workspace)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        if p.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s == "target"
                || s == "node_modules"
                || s == ".git"
                || s == "deps"
                || s == ".routify"
                || s == ".yarn"
                || s == ".vscode"
                || s == "log"
                || s == "dist"
        }) {
            continue;
        }
        if let Ok(md) = p.metadata() {
            if md.len() > 1_500_000 {
                continue;
            }
        }
        let content = match tokio::fs::read_to_string(p).await {
            Ok(s) => s,
            Err(_) => continue,
        };
        for (idx, line) in content.lines().enumerate() {
            if let Some(pos) = line.find(&q.q) {
                let rel = match p.strip_prefix(&state.workspace) {
                    Ok(r) => r.to_string_lossy().to_string(),
                    Err(_) => continue,
                };
                results.push(Hit {
                    path: rel,
                    line: idx + 1,
                    col: pos + 1,
                    text: line.to_string(),
                });
                if results.len() >= max {
                    return Json(results);
                }
            }
        }
    }
    Json(results)
}

async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_loop(state, socket))
}

type WsSink = futures::stream::SplitSink<WebSocket, Message>;
type SinkHandle = Arc<tokio::sync::Mutex<WsSink>>;

async fn send_server_msg(sink: &SinkHandle, msg: ServerMsg) {
    let text = match serde_json::to_string(&msg) {
        Ok(t) => t,
        Err(_) => return,
    };
    let mut guard = sink.lock().await;
    let _ = guard.send(Message::Text(text)).await;
}

// Session state to hold all stateful data
struct SessionState {
    lsp: Option<lsp_bridge::LspProcess>,
    runner: Option<Runner>,
    runner_reader_out: Option<tokio::task::JoinHandle<()>>,
    runner_reader_err: Option<tokio::task::JoinHandle<()>>,
    term: Option<Term>,
}

struct Runner {
    child: tokio::process::Child,
    stdin: Option<tokio::process::ChildStdin>,
}

struct Term {
    master: Box<dyn portable_pty::MasterPty + Send>,
    writer: Box<dyn std::io::Write + Send>,
    child: Box<dyn portable_pty::Child + Send>,
    reader: Option<std::thread::JoinHandle<()>>,
}

impl SessionState {
    fn new() -> Self {
        Self {
            lsp: None,
            runner: None,
            runner_reader_out: None,
            runner_reader_err: None,
            term: None,
        }
    }

    async fn cleanup(&mut self) {
        if let Some(proc) = self.lsp.take() {
            proc.shutdown().await;
        }
        if let Some(mut r) = self.runner.take() {
            let _ = r.child.kill().await;
            let _ = r.child.wait().await;
        }
        if let Some(h) = self.runner_reader_out.take() {
            let _ = h.abort();
        }
        if let Some(h) = self.runner_reader_err.take() {
            let _ = h.abort();
        }
        if let Some(mut t) = self.term.take() {
            let _ = t.child.kill();
            let _ = t.child.wait();
            if let Some(h) = t.reader {
                let _ = h.join();
            }
        }
    }
}

// ============================================================================
// LSP HANDLERS
// ============================================================================

async fn handle_lsp_spawn(
    session: &mut SessionState,
    sink: &SinkHandle,
    lang: String,
    workspace: &std::path::Path,
    tx_lsp: &tokio::sync::mpsc::UnboundedSender<String>,
) {
    let (cmd, args) = match lang.as_str() {
        "ts" | "typescript" => (
            "typescript-language-server".to_string(),
            vec!["--stdio".to_string()],
        ),
        "rust" => ("rust-analyzer".to_string(), vec![]),
        _ => {
            send_server_msg(
                sink,
                ServerMsg::Error {
                    message: format!("unknown lang {}", lang),
                },
            )
            .await;
            return;
        }
    };

    // Shutdown existing LSP if any
    if let Some(proc) = session.lsp.take() {
        proc.shutdown().await;
    }

    match lsp_bridge::LspProcess::spawn(&cmd, &args, Some(workspace.to_string_lossy().as_ref()))
        .await
    {
        Ok(mut proc) => {
            let mut rx = match proc.take_rx() {
                Some(r) => r,
                None => {
                    send_server_msg(
                        sink,
                        ServerMsg::Error {
                            message: "rx already taken".into(),
                        },
                    )
                    .await;
                    return;
                }
            };

            let tx_clone = tx_lsp.clone();
            tokio::spawn(async move {
                while let Some(m) = rx.recv().await {
                    let _ = tx_clone.send(m);
                }
            });

            let _ = proc.send(&lsp_bridge::initialize_message()).await;
            let _ = proc.send(&lsp_bridge::initialized_notification()).await;
            session.lsp = Some(proc);

            send_server_msg(
                sink,
                ServerMsg::Info {
                    message: format!("LSP spawned: {}", cmd),
                },
            )
            .await;
        }
        Err(e) => {
            send_server_msg(
                sink,
                ServerMsg::Error {
                    message: format!("LSP spawn failed: {}", e),
                },
            )
            .await;
        }
    }
}

async fn handle_lsp_send(session: &mut SessionState, sink: &SinkHandle, payload: String) {
    if let Some(proc) = &mut session.lsp {
        let _ = proc.send(&payload).await;
    } else {
        send_server_msg(
            sink,
            ServerMsg::Error {
                message: "LSP not spawned".into(),
            },
        )
        .await;
    }
}

// ============================================================================
// RUNNER HANDLERS
// ============================================================================

async fn spawn_runner(cmdline: &str, cwd: &std::path::Path) -> Result<Runner, String> {
    let mut cmd = if cfg!(windows) {
        let mut c = tokio::process::Command::new("cmd");
        c.arg("/C");
        c.arg(cmdline);
        c
    } else {
        let mut c = tokio::process::Command::new("sh");
        c.arg("-lc");
        c.arg(cmdline);
        c
    };

    cmd.current_dir(cwd);
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match cmd.spawn() {
        Ok(mut child) => {
            let stdin = child.stdin.take();
            Ok(Runner { child, stdin })
        }
        Err(e) => Err(e.to_string()),
    }
}

async fn handle_runner_start(
    session: &mut SessionState,
    sink: &SinkHandle,
    cmdline: &str,
    cwd: &std::path::Path,
) {
    if session.runner.is_some() {
        send_server_msg(
            sink,
            ServerMsg::Info {
                message: "runner already started".into(),
            },
        )
        .await;
        return;
    }

    match spawn_runner(cmdline, cwd).await {
        Ok(mut r) => {
            let mut out_pipe = r.child.stdout.take();
            let mut err_pipe = r.child.stderr.take();

            // Stdout reader
            let sink_for_runner_out = sink.clone();
            let t_out = tokio::spawn(async move {
                if let Some(mut out) = out_pipe.take() {
                    let mut buf = [0u8; 4096];
                    loop {
                        match out.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                let chunk = String::from_utf8_lossy(&buf[..n]).to_string();
                                send_server_msg(
                                    &sink_for_runner_out,
                                    ServerMsg::RunnerLog { chunk },
                                )
                                .await;
                            }
                            Err(_) => break,
                        }
                    }
                }
            });

            // Stderr reader
            let sink_for_runner_err = sink.clone();
            let t_err = tokio::spawn(async move {
                if let Some(mut er) = err_pipe.take() {
                    let mut buf = [0u8; 4096];
                    loop {
                        match er.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                let chunk = String::from_utf8_lossy(&buf[..n]).to_string();
                                send_server_msg(
                                    &sink_for_runner_err,
                                    ServerMsg::RunnerErr { chunk },
                                )
                                .await;
                            }
                            Err(_) => break,
                        }
                    }
                }
            });

            session.runner_reader_out = Some(t_out);
            session.runner_reader_err = Some(t_err);
            session.runner = Some(r);

            send_server_msg(
                sink,
                ServerMsg::Info {
                    message: format!("runner started: {}", cmdline),
                },
            )
            .await;
        }
        Err(e) => {
            send_server_msg(
                sink,
                ServerMsg::Error {
                    message: format!("runner spawn failed: {}", e),
                },
            )
            .await;
        }
    }
}

async fn handle_runner_stop(session: &mut SessionState, sink: &SinkHandle) {
    if let Some(mut r) = session.runner.take() {
        let _ = r.child.kill().await;
        let code = match r.child.wait().await {
            Ok(s) => s.code().unwrap_or(-1),
            Err(_) => -1,
        };
        send_server_msg(sink, ServerMsg::RunnerExit { code }).await;
    }
    if let Some(h) = session.runner_reader_out.take() {
        let _ = h.abort();
    }
    if let Some(h) = session.runner_reader_err.take() {
        let _ = h.abort();
    }
}

async fn handle_runner_restart(
    session: &mut SessionState,
    sink: &SinkHandle,
    cmdline: &str,
    cwd: &std::path::Path,
) {
    // Stop existing runner
    handle_runner_stop(session, sink).await;

    // Start new runner
    handle_runner_start(session, sink, cmdline, cwd).await;
}

async fn handle_runner_input(session: &mut SessionState, sink: &SinkHandle, data: String) {
    if let Some(r) = &mut session.runner {
        if let Some(stdin) = &mut r.stdin {
            let _ = stdin.write_all(data.as_bytes()).await;
            let _ = stdin.flush().await;
        } else {
            send_server_msg(
                sink,
                ServerMsg::Error {
                    message: "runner stdin closed".into(),
                },
            )
            .await;
        }
    } else {
        send_server_msg(
            sink,
            ServerMsg::Error {
                message: "runner not started".into(),
            },
        )
        .await;
    }
}

// ============================================================================
// TERMINAL HANDLERS
// ============================================================================

async fn handle_terminal_start(
    session: &mut SessionState,
    sink: &SinkHandle,
    state: &AppState,
    cols: u16,
    rows: u16,
) {
    if session.term.is_some() {
        send_server_msg(
            sink,
            ServerMsg::Info {
                message: "terminal already started".into(),
            },
        )
        .await;
        return;
    }

    let pty_system = portable_pty::native_pty_system();
    let size = portable_pty::PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    };

    match pty_system.openpty(size) {
        Ok(pair) => {
            let mut cmd = build_terminal_command(state, sink).await;
            cmd.cwd(state.workspace.clone());

            match pair.slave.spawn_command(cmd) {
                Ok(child) => {
                    let mut reader = match pair.master.try_clone_reader() {
                        Ok(r) => r,
                        Err(e) => {
                            send_server_msg(
                                sink,
                                ServerMsg::Error {
                                    message: format!("pty reader: {}", e),
                                },
                            )
                            .await;
                            return;
                        }
                    };

                    let writer = match pair.master.take_writer() {
                        Ok(w) => w,
                        Err(e) => {
                            send_server_msg(
                                sink,
                                ServerMsg::Error {
                                    message: format!("pty writer: {}", e),
                                },
                            )
                            .await;
                            return;
                        }
                    };

                    let master = pair.master;
                    let sink_for_term = sink.clone();
                    let rt = tokio::runtime::Handle::current();

                    let reader_thread = std::thread::spawn(move || {
                        let mut buf = [0u8; 8192];
                        loop {
                            match reader.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => {
                                    let chunk = String::from_utf8_lossy(&buf[..n]).to_string();
                                    let _ = rt.block_on(async {
                                        send_server_msg(
                                            &sink_for_term,
                                            ServerMsg::TerminalData { chunk },
                                        )
                                        .await;
                                    });
                                }
                                Err(_) => break,
                            }
                        }
                    });

                    session.term = Some(Term {
                        master,
                        writer,
                        child,
                        reader: Some(reader_thread),
                    });

                    let mode_label = match (state.sandbox, state.allow_net) {
                        (SandboxMode::Bwrap, true) => "bwrap(net=ON)",
                        (SandboxMode::Bwrap, false) => "bwrap(net=OFF)",
                        _ => "unsandboxed",
                    };

                    send_server_msg(
                        sink,
                        ServerMsg::Info {
                            message: format!("terminal started ({})", mode_label),
                        },
                    )
                    .await;
                }
                Err(e) => {
                    send_server_msg(
                        sink,
                        ServerMsg::Error {
                            message: format!("spawn shell: {}", e),
                        },
                    )
                    .await;
                }
            }
        }
        Err(e) => {
            send_server_msg(
                sink,
                ServerMsg::Error {
                    message: format!("openpty: {}", e),
                },
            )
            .await;
        }
    }
}

async fn build_terminal_command(
    state: &AppState,
    sink: &SinkHandle,
) -> portable_pty::CommandBuilder {
    if state.sandbox == SandboxMode::Bwrap && !cfg!(windows) {
        build_sandboxed_terminal_command(state, sink).await
    } else {
        build_unsandboxed_terminal_command()
    }
}

async fn build_sandboxed_terminal_command(
    state: &AppState,
    sink: &SinkHandle,
) -> portable_pty::CommandBuilder {
    let mut c = portable_pty::CommandBuilder::new("bwrap");

    // Network configuration
    if state.allow_net {
        c.arg("--unshare-user");
        c.arg("--unshare-ipc");
        c.arg("--unshare-pid");
        c.arg("--unshare-uts");
        c.arg("--unshare-cgroup");
    } else {
        c.arg("--unshare-all");
    }

    // c.arg("--new-session");
    c.arg("--die-with-parent");

    // Workspace as root
    let ws = state.workspace.to_string_lossy().to_string();
    c.arg("--bind");
    c.arg(&ws);
    c.arg("/");

    // Minimal fs
    c.arg("--dev");
    c.arg("/dev");
    c.arg("--proc");
    c.arg("/proc");
    c.arg("--tmpfs");
    c.arg("/tmp");

    // Fresh /bin
    c.arg("--tmpfs");
    c.arg("/bin");

    if let Some(bb) = &state.busybox {
        // Bind busybox
        let bb_os = bb.to_string_lossy().to_string();
        c.arg("--ro-bind");
        c.arg(&bb_os);
        c.arg("/bin/busybox");

        // Tools directory
        let tools_bin = "/tools/bin";
        c.arg("--dir");
        c.arg(tools_bin);

        if let Some(tools_dir) = &state.tools_dir {
            c.arg("--ro-bind");
            c.arg(tools_dir.to_string_lossy().as_ref());
            c.arg(tools_bin);
        }

        // Environment
        let path_val = format!("/bin:{}", tools_bin);
        c.arg("--setenv");
        c.arg("PATH");
        c.arg(&path_val);
        c.arg("--setenv");
        c.arg("HOME");
        c.arg("/");

        // Symlink busybox applets
        for app in [
            "curl", "wget", "bash", "sh", "ls", "cat", "vi", "grep", "less", "tail", "head", "sed",
            "awk", "find", "mkdir", "rm", "mv", "cp", "echo", "pwd", "touch", "tee", "cut", "sort",
            "uniq", "wc", "stat", "du", "df",
        ]
        .iter()
        {
            c.arg("--symlink");
            c.arg("/bin/busybox");
            c.arg(format!("/bin/{}", app));
        }

        add_env_and_claude_bindings(&mut c, state, sink);
        configure_shell_command(&mut c, state);
    } else {
        // No busybox - bind system binaries
        c.arg("--ro-bind");
        c.arg("/bin/bash");
        c.arg("/bin/sh");
        c.arg("--ro-bind");
        c.arg("/lib");
        c.arg("/lib");
        if std::path::Path::new("/lib64").exists() {
            c.arg("--ro-bind");
            c.arg("/lib64");
            c.arg("/lib64");
        }
        c.arg("--ro-bind");
        c.arg("/usr");
        c.arg("/usr");

        add_env_and_claude_bindings(&mut c, state, sink);
        configure_shell_command(&mut c, state);
    }

    c.cwd(state.workspace.clone());
    c
}

fn build_unsandboxed_terminal_command() -> portable_pty::CommandBuilder {
    let mut c = portable_pty::CommandBuilder::new(std::env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(windows) {
            "cmd".into()
        } else {
            "/bin/bash".into()
        }
    }));
    if !cfg!(windows) {
        c.arg("-l");
    }
    c
}

async fn handle_terminal_input(session: &mut SessionState, data: String) {
    if let Some(t) = &mut session.term {
        use std::io::Write;
        let _ = t.writer.write_all(data.as_bytes());
    }
}

async fn handle_terminal_resize(session: &mut SessionState, cols: u16, rows: u16) {
    if let Some(t) = &mut session.term {
        let _ = t.master.resize(portable_pty::PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        });
    }
}

async fn handle_terminal_stop(session: &mut SessionState, sink: &SinkHandle) {
    if let Some(mut t) = session.term.take() {
        let _ = t.child.kill();
        let _ = t.child.wait();
        if let Some(h) = t.reader.take() {
            let _ = h.join();
        }
        send_server_msg(sink, ServerMsg::TerminalExit { code: 0 }).await;
    }
}

// ============================================================================
// UTILITY HANDLERS
// ============================================================================

fn handle_cargo(
    sink: SinkHandle,
    workspace: std::path::PathBuf,
    sub: String,
    extra: Option<String>,
) {
    tokio::spawn(async move {
        let mut cmd = tokio::process::Command::new("cargo");
        cmd.arg(sub);
        if let Some(e) = extra {
            for part in e.split_whitespace() {
                cmd.arg(part);
            }
        }
        cmd.current_dir(workspace);
        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(mut child) => {
                if let Some(mut out) = child.stdout.take() {
                    let mut buf = [0u8; 4096];
                    loop {
                        match out.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => {
                                let chunk = String::from_utf8_lossy(&buf[..n]).to_string();
                                send_server_msg(&sink, ServerMsg::CargoLog { chunk }).await;
                            }
                            Err(_) => break,
                        }
                    }
                }
                let code = match child.wait().await {
                    Ok(s) => s.code().unwrap_or(1),
                    Err(_) => 1,
                };
                send_server_msg(&sink, ServerMsg::CargoExit { code }).await;
            }
            Err(e) => {
                send_server_msg(
                    &sink,
                    ServerMsg::Error {
                        message: format!("cargo spawn failed: {}", e),
                    },
                )
                .await;
            }
        }
    });
}

fn handle_run_once(sink: SinkHandle, workspace: std::path::PathBuf, cmd: String) {
    tokio::spawn(async move {
        let mut p = tokio::process::Command::new(if cfg!(windows) { "cmd" } else { "sh" });
        if cfg!(windows) {
            p.arg("/C");
            p.arg(&cmd);
        } else {
            p.arg("-lc");
            p.arg(&cmd);
        }
        p.current_dir(&workspace)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match p.spawn() {
            Ok(mut child) => {
                let mut out = child.stdout.take();
                let mut err = child.stderr.take();
                let s1 = sink.clone();
                let t1 = tokio::spawn(async move {
                    if let Some(mut r) = out.take() {
                        let mut buf = [0u8; 4096];
                        loop {
                            match r.read(&mut buf).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    send_server_msg(
                                        &s1,
                                        ServerMsg::RunOnceOutput {
                                            chunk: String::from_utf8_lossy(&buf[..n]).into(),
                                        },
                                    )
                                    .await;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                });
                let s2 = sink.clone();
                let t2 = tokio::spawn(async move {
                    if let Some(mut r) = err.take() {
                        let mut buf = [0u8; 4096];
                        loop {
                            match r.read(&mut buf).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    send_server_msg(
                                        &s2,
                                        ServerMsg::RunOnceOutput {
                                            chunk: String::from_utf8_lossy(&buf[..n]).into(),
                                        },
                                    )
                                    .await;
                                }
                                Err(_) => break,
                            }
                        }
                    }
                });
                let code = match child.wait().await {
                    Ok(s) => s.code().unwrap_or(1),
                    Err(_) => 1,
                };
                let _ = t1.await;
                let _ = t2.await;
                send_server_msg(&sink, ServerMsg::RunOnceExit { code }).await;
            }
            Err(e) => {
                send_server_msg(
                    &sink,
                    ServerMsg::RunOnceOutput {
                        chunk: format!("spawn error: {e}\n"),
                    },
                )
                .await;
                send_server_msg(&sink, ServerMsg::RunOnceExit { code: 127 }).await;
            }
        }
    });
}

fn handle_ai_query(
    sink: SinkHandle,
    workspace: std::path::PathBuf,
    sandbox: SandboxMode,
    prompt: String,
) {
    tokio::spawn(async move {
        let claude_cmd = if sandbox == SandboxMode::Bwrap {
            "/tools/bin/claude".to_string()
        } else {
            "claude".to_string()
        };

        let mut cmd = tokio::process::Command::new(&claude_cmd);
        cmd.arg(&prompt);
        cmd.current_dir(&workspace);
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        send_server_msg(
            &sink,
            ServerMsg::Info {
                message: format!(
                    "Querying Claude: {}",
                    prompt.chars().take(50).collect::<String>()
                ),
            },
        )
        .await;

        match cmd.spawn() {
            Ok(mut child) => {
                let mut response = String::new();
                if let Some(mut stdout) = child.stdout.take() {
                    let mut buf = Vec::new();
                    if stdout.read_to_end(&mut buf).await.is_ok() {
                        response = String::from_utf8_lossy(&buf).to_string();
                    }
                }

                let mut stderr_output = String::new();
                if let Some(mut stderr) = child.stderr.take() {
                    let mut buf = Vec::new();
                    if stderr.read_to_end(&mut buf).await.is_ok() {
                        stderr_output = String::from_utf8_lossy(&buf).to_string();
                    }
                }

                let code = match child.wait().await {
                    Ok(s) => s.code().unwrap_or(1),
                    Err(_) => 1,
                };

                if code == 0 && !response.is_empty() {
                    send_server_msg(
                        &sink,
                        ServerMsg::AiResponse {
                            provider: "claude".to_string(),
                            model: "claude-sonnet-4".to_string(),
                            response,
                        },
                    )
                    .await;
                } else {
                    let error_msg = if !stderr_output.is_empty() {
                        stderr_output
                    } else if !response.is_empty() {
                        response
                    } else {
                        format!("Claude command failed with exit code {}", code)
                    };
                    send_server_msg(
                        &sink,
                        ServerMsg::Error {
                            message: format!("Claude error: {}", error_msg),
                        },
                    )
                    .await;
                }
            }
            Err(e) => {
                send_server_msg(&sink, ServerMsg::Error {
                    message: format!("Failed to spawn Claude: {}. Make sure Claude CLI is installed and accessible.", e),
                }).await;
            }
        }
    });
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClientMsg {
    LspSpawn { lang: String },
    LspSend { payload: String },
    Cargo { sub: String, extra: Option<String> },
    RunnerStart,
    RunnerStop,
    RunnerRestart,
    RunnerInput { data: String },
    TerminalStart { cols: u16, rows: u16 },
    TerminalInput { data: String },
    TerminalResize { cols: u16, rows: u16 },
    TerminalStop,
    RunOnce { cmd: String },
    AiQuery { prompt: String },
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ServerMsg {
    Info {
        message: String,
    },
    Error {
        message: String,
    },
    LspRecv {
        payload: String,
    },
    CargoLog {
        chunk: String,
    },
    CargoExit {
        code: i32,
    },
    RunnerLog {
        chunk: String,
    }, // stdout
    RunnerErr {
        chunk: String,
    }, // <-- NEW: stderr
    RunnerExit {
        code: i32,
    },
    TerminalData {
        chunk: String,
    },
    TerminalExit {
        code: i32,
    },
    RunOnceOutput {
        chunk: String,
    },
    RunOnceExit {
        code: i32,
    },
    FileChanged {
        path: String,
    }, // for the watcher below
    AiResponse {
        provider: String,
        model: String,
        response: String,
    },
}

async fn ws_loop(state: AppState, socket: WebSocket) {
    let (sink, mut stream) = socket.split();
    let sink_arc: SinkHandle = Arc::new(tokio::sync::Mutex::new(sink));

    // LSP message channel
    let (tx_lsp, mut rx_lsp) = tokio::sync::mpsc::unbounded_channel::<String>();

    // File watcher setup
    let (fw_tx, _fw_rx) = tokio::sync::broadcast::channel::<String>(1024);
    let _watcher = spawn_file_watcher(state.workspace.clone(), fw_tx.clone())
        .map_err(|e| {
            eprintln!("watcher error {e}");
            e
        })
        .ok();

    // Spawn file watcher broadcaster
    let mut fw_sub = fw_tx.subscribe();
    let sink_for_files = sink_arc.clone();
    tokio::spawn(async move {
        while let Ok(path) = fw_sub.recv().await {
            send_server_msg(&sink_for_files, ServerMsg::FileChanged { path }).await;
        }
    });

    // Spawn LSP message forwarder
    let sink_for_lsp = sink_arc.clone();
    let forward_lsp = tokio::spawn(async move {
        while let Some(msg) = rx_lsp.recv().await {
            send_server_msg(&sink_for_lsp, ServerMsg::LspRecv { payload: msg }).await;
        }
    });

    // Initialize session state
    let mut session = SessionState::new();

    // Main message loop
    while let Some(Ok(msg)) = stream.next().await {
        if let Message::Text(txt) = msg {
            match serde_json::from_str::<ClientMsg>(&txt) {
                // LSP handlers
                Ok(ClientMsg::LspSpawn { lang }) => {
                    handle_lsp_spawn(&mut session, &sink_arc, lang, &state.workspace, &tx_lsp)
                        .await;
                }
                Ok(ClientMsg::LspSend { payload }) => {
                    handle_lsp_send(&mut session, &sink_arc, payload).await;
                }

                // Cargo handler
                Ok(ClientMsg::Cargo { sub, extra }) => {
                    handle_cargo(sink_arc.clone(), state.workspace.clone(), sub, extra);
                }

                // Runner handlers
                Ok(ClientMsg::RunnerStart) => {
                    let Some(cmdline) = &state.runner_cmd else {
                        send_server_msg(
                            &sink_arc,
                            ServerMsg::Error {
                                message: "no --runner configured".into(),
                            },
                        )
                        .await;
                        continue;
                    };
                    handle_runner_start(&mut session, &sink_arc, cmdline, &state.workspace).await;
                }
                Ok(ClientMsg::RunnerStop) => {
                    handle_runner_stop(&mut session, &sink_arc).await;
                }
                Ok(ClientMsg::RunnerRestart) => {
                    let Some(cmdline) = &state.runner_cmd else {
                        send_server_msg(
                            &sink_arc,
                            ServerMsg::Error {
                                message: "no --runner configured".into(),
                            },
                        )
                        .await;
                        continue;
                    };
                    handle_runner_restart(&mut session, &sink_arc, cmdline, &state.workspace).await;
                }
                Ok(ClientMsg::RunnerInput { data }) => {
                    handle_runner_input(&mut session, &sink_arc, data).await;
                }

                // Terminal handlers
                Ok(ClientMsg::TerminalStart { cols, rows }) => {
                    handle_terminal_start(&mut session, &sink_arc, &state, cols, rows).await;
                }
                Ok(ClientMsg::TerminalInput { data }) => {
                    handle_terminal_input(&mut session, data).await;
                }
                Ok(ClientMsg::TerminalResize { cols, rows }) => {
                    handle_terminal_resize(&mut session, cols, rows).await;
                }
                Ok(ClientMsg::TerminalStop) => {
                    handle_terminal_stop(&mut session, &sink_arc).await;
                }

                // Utility handlers
                Ok(ClientMsg::RunOnce { cmd }) => {
                    handle_run_once(sink_arc.clone(), state.workspace.clone(), cmd);
                }
                Ok(ClientMsg::AiQuery { prompt }) => {
                    handle_ai_query(
                        sink_arc.clone(),
                        state.workspace.clone(),
                        state.sandbox,
                        prompt,
                    );
                }

                // Error handling
                Err(e) => {
                    send_server_msg(
                        &sink_arc,
                        ServerMsg::Error {
                            message: format!("bad client msg: {}", e),
                        },
                    )
                    .await;
                }
            }
        }
    }

    // Cleanup on disconnect
    session.cleanup().await;
    let _ = forward_lsp.abort();
}
