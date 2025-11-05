<script>
	import { onMount } from "svelte";
  import Main from "../components/Main.svelte";
import Sidebar from "../components/Sidebar.svelte";
import TopBar from "../components/TopBar.svelte";
 import * as monaco from "monaco-editor";
import {Terminal} from "xterm";
  import {FitAddon} from "xterm-addon-fit";
  import { WebLinksAddon } from 'xterm-addon-web-links';


onMount(()=>{
 let editor, ws, currentPath = null;
let terminal, fitAddon, termStarted = false;
const statusEl = document.getElementById('status');
const wsPathEl = document.getElementById('ws-path');
const logEl = document.getElementById('log');
const termEl = document.getElementById('termlog');
const filesEl = document.getElementById('files');
const xtermEl = document.getElementById('xterm');
const previewEl = document.getElementById('preview');


function log(el, s) { el.textContent += s + "\\n"; el.scrollTop = el.scrollHeight; }
function setStatus(s) { statusEl.textContent = s; }
function setWorkspacePath() { wsPathEl.textContent = location.host; }

function connectWS() {
  ws = new WebSocket(`ws://127.0.0.1:8788/ws`);
  ws.onopen = () => setStatus('WS connected');
  ws.onmessage = (ev) => {
    try {
      const msg = JSON.parse(ev.data);
      if (msg.type === 'info') setStatus(msg.message);
      if (msg.type === 'error') log(logEl, '[error] ' + msg.message);
      if (msg.type === 'lsp_recv') log(logEl, '[lsp] ' + msg.payload.slice(0, 500));
      if (msg.type === 'cargo_log') log(logEl, msg.chunk);
      if (msg.type === 'cargo_exit') log(logEl, 'cargo exit code ' + msg.code);
      if (msg.type === 'runner_log') log(termEl, msg.chunk);
      if (msg.type === 'runner_exit') log(termEl, 'runner exit code ' + msg.code);
      if (msg.type === 'terminal_data') { if (terminal) terminal.write(msg.chunk); }
    } catch (e) { log(logEl, 'bad message: ' + e); }
  };
  ws.onclose = () => setStatus('WS closed');
}

function setTab(tab) {
  for (const t of document.querySelectorAll('.tab')) t.classList.toggle('active', t.dataset.tab === tab);
  logEl.style.display = (tab === 'lsp') ? 'block' : 'none';
  xtermEl.style.display = (tab === 'terminal') ? 'block' : 'none';
  termEl.style.display = (tab === 'runner') ? 'block' : 'none';
  if (tab === 'terminal') ensureTerminalStarted();
}
for (const t of document.querySelectorAll('.tab')) { t.onclick = () => setTab(t.dataset.tab); }
document.getElementById('btn-clear').onclick = () => {
  if (logEl.style.display !== 'none') logEl.textContent='';
  if (termEl.style.display !== 'none') termEl.textContent='';
  if (xtermEl.style.display !== 'none' && terminal) terminal.clear();
};

document.getElementById('runner-start').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_start' })); }
document.getElementById('runner-restart').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_restart' })); }
document.getElementById('runner-stop').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_stop' })); }

document.getElementById('toggle-sidebar').onclick = () => {
  const sb = document.getElementById('sidebar');
  const container = document.getElementById('container');
  if (sb.classList.contains('hidden')) { sb.classList.remove('hidden'); container.style.gridTemplateColumns = '300px 1fr'; }
  else { sb.classList.add('hidden'); container.style.gridTemplateColumns = '0px 1fr'; }
};




function renderMarkdown() {
  if (!currentPath) return;
  const text = editor.getValue();
  const html = DOMPurify.sanitize(marked.parse(text));
  previewEl.innerHTML = html;
}

async function listFiles() {
  const res = await fetch('http://127.0.0.1:8788/api/list');
  const arr = await res.json();
  const root = {};
  for (const p of arr) {
    const parts = p.split('/');
    let node = root;
    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const isLast = i === parts.length - 1;
      if (!node[part]) node[part] = { _files: {}, _open: i < 2, _isDir: !isLast };
      node = node[part]._files;
      if (isLast) { node[part] = { _files: {}, _open: false, _isDir: false }; }
    }
  }
  filesEl.innerHTML = '';
  renderTree(filesEl, root, []);

  const filterInput = document.getElementById('file-search');
  filterInput.oninput = () => { filesEl.innerHTML = ''; renderTree(filesEl, root, filterInput.value.toLowerCase().split(/\\s+/).filter(Boolean)); };
}

function renderTree(rootEl, tree, filters) {
  const ul = document.createElement('ul');
  ul.style.listStyle = 'none';
  ul.style.paddingLeft = '10px';

  for (const [name, meta] of Object.entries(tree)) {
    if (name === '_files') continue;
    const isDir = meta._isDir ?? true;
    const li = document.createElement('li');
    const row = document.createElement('div');
    row.className = 'node';
    const twisty = document.createElement('span');
    twisty.className = 'twisty';
    twisty.textContent = isDir ? (meta._open ? '▼' : '▶') : '•';
    const fname = document.createElement('span');
    fname.className = 'fname';
    fname.textContent = name;

    row.appendChild(twisty);
    row.appendChild(fname);
    li.appendChild(row);
    ul.appendChild(li);

    let passes = true;
    if (filters.length) { const low = name.toLowerCase(); passes = filters.every(f => low.includes(f)); }
    if (!passes) li.style.display = 'none';

    if (isDir) {
      const childRoot = document.createElement('div');
      childRoot.style.paddingLeft = '14px';
      li.appendChild(childRoot);
      const open = !!meta._open;
      childRoot.style.display = open ? 'block' : 'none';
      row.onclick = () => { meta._open = !meta._open; twisty.textContent = meta._open ? '▼' : '▶'; childRoot.style.display = meta._open ? 'block' : 'none'; };
      renderTree(childRoot, meta._files, filters);
    } else { row.onclick = () => openFile(pathJoin(li)); }

    function pathJoin(liNode) {
      const names = [];
      let cur = liNode;
      while (cur && cur.parentElement && cur !== rootEl) {
        const row = cur.querySelector(':scope > .node .fname'); if (row) names.push(row.textContent);
        cur = cur.parentElement.closest('li');
      }
      return names.reverse().join('/');
    }
  }
  rootEl.appendChild(ul);
}

function detectLanguage(path, content) {
  const lower = path.toLowerCase();
  if (lower.endsWith('.rs')) return 'rust';
  if (lower.endsWith('.ts') || lower.endsWith('.tsx')) return 'typescript';
  if (lower.endsWith('.js') || lower.endsWith('.jsx')) return 'javascript';
  if (lower.endsWith('.json')) return 'json';
  if (lower.endsWith('.toml')) return 'toml';
  if (lower.endsWith('.yml') || lower.endsWith('.yaml')) return 'yaml';
  if (lower.endsWith('.md') || lower.endsWith('.markdown')) return 'markdown';
  if (lower.endsWith('.css')) return 'css';
  if (lower.endsWith('.html') || lower.endsWith('.htm')) return 'html';
  if (lower.endsWith('.rest')) return 'http';
  if (lower.endsWith('.env')) return 'ini';
  if (lower.endsWith('.sh') || lower.endsWith('.bash') || lower.endsWith('.zsh') || lower.endsWith('.fish')) return 'shell';
  if (content.startsWith('#!')) {
    if (content.includes('bash') || content.includes('zsh') || content.includes('sh') || content.includes('fish') || content.includes('dash')) return 'shell';
    if (content.includes('python')) return 'python';
  }
  return 'plaintext';
}

async function openFile(p) {
  const url = 'http://127.0.0.1:8788/api/open?' + new URLSearchParams({ path: p });
  const res = await fetch(url);
  const js = await res.json();
  if (js.ok) {
    currentPath = p;
    const content = js.content;
    editor.setValue(content);
    const lang = detectLanguage(p, content);
    try { monaco.editor.setModelLanguage(editor.getModel(), lang); } catch {}
    if (lang === 'markdown' && previewEl.classList.contains('hidden') === false) renderMarkdown();
    if (lang !== 'markdown' && !previewEl.classList.contains('hidden')) togglePreview();
  } else { log(logEl, 'open failed: ' + js.error); }
}

document.addEventListener('mouseup', async () => {
  if (!editor) return;
  const sel = editor.getSelection();
  if (!sel || sel.isEmpty()) return;
  const model = editor.getModel();
  const text = model.getValueInRange(sel);
  if (text && navigator.clipboard) {
    try { await navigator.clipboard.writeText(text); setStatus('copied selection'); } catch {}
  }
});

// setWorkspacePath();
// connectWS();
// listFiles();



function ensureTerminalStarted() {
  if (termStarted) return;
  termStarted = true;
  terminal = new Terminal({ cursorBlink: true, fontSize: 13, theme: { background: '#0b0e14', foreground: '#cbd5e1' } });
  fitAddon = new FitAddon();
  const linkAddon = new WebLinksAddon();
  terminal.loadAddon(fitAddon); terminal.loadAddon(linkAddon);
  terminal.open(xtermEl);
  setTimeout(() => { try { fitAddon.fit(); } catch {} }, 0);
  terminal.onData((d) => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'terminal_input', data: d })); });
  if (ws && ws.readyState === WebSocket.OPEN) {
    const cols = Math.max(80, Math.floor(xtermEl.clientWidth / 9));
    const rows = Math.max(24, Math.floor(xtermEl.clientHeight / 18));
    ws.send(JSON.stringify({ type: 'terminal_start', cols, rows }));
  }
  window.addEventListener('resize', () => {
    if (fitAddon) {
      try { fitAddon.fit(); } catch {}
      if (ws && ws.readyState === WebSocket.OPEN) {
        const cols = Math.max(40, Math.floor(xtermEl.clientWidth / 9));
        const rows = Math.max(10, Math.floor(xtermEl.clientHeight / 18));
        ws.send(JSON.stringify({ type: 'terminal_resize', cols, rows }));
      }
    }
  });
}

// require(['vs/editor/editor.main', 'vs/basic-languages/toml/toml', 'vs/basic-languages/shell/shell', 'vs/basic-languages/http/http'], function () {

     editor = monaco.editor.create(document.getElementById('editor'), {
    value: '// Select a file on the left to open it.',
    language: 'plaintext',
    automaticLayout: true,
    theme: 'vs-dark',
    minimap: { enabled: false },
    fontLigatures: true,
    fontSize: 14,
  });
  editor.onDidChangeModelContent(() => {
    if (currentPath && (currentPath.endsWith('.md') || currentPath.endsWith('.markdown')) && !previewEl.classList.contains('hidden')) {
      renderMarkdown();
    }
  });
  setWorkspacePath();
  connectWS();
  listFiles();
});

</script>

<TopBar/>
  <div id="container">
   <Sidebar/>
   <Main/>
  </div>
<style>

    #container { display: grid; grid-template-columns: 300px 1fr; height: calc(100% - 44px); }
   
</style>