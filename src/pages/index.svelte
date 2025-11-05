<script>

</script>

 <div id="topbar">
    <button id="toggle-sidebar">☰</button>
    <button id="btn-rust">Rust LSP</button>
    <button id="btn-ts">TS LSP</button>
    <button id="btn-check">cargo check</button>
    <button id="btn-save">Save</button>
    <button id="btn-clear">Clear</button>
    <button id="btn-preview">Preview</button>
    <span class="tag">Workspace</span><span id="ws-path" class="dim"></span>
    <span id="status" class="status">—</span>
  </div>
  <div id="container">
    <div id="sidebar">
      <div id="sidebar-top">
        <input id="file-search" type="text" placeholder="Search files by name…" />
        <button id="btn-search">Find in files</button>
      </div>
      <ul id="files"></ul>
    </div>
    <div id="main">
      <div id="editor-area">
        <div id="editor"></div>
        <div id="preview" class="hidden"></div>
      </div>
      <div id="bottom">
        <div id="tabs">
          <div class="tab active" data-tab="lsp">LSP</div>
          <div class="tab" data-tab="terminal">Terminal</div>
          <div class="tab" data-tab="runner">Runner</div>
          <button id="runner-start">Start</button>
          <button id="runner-restart">Restart</button>
          <button id="runner-stop">Stop</button>
        </div>
        <div id="log"></div>
        <div id="xterm"></div>
        <div id="termlog"></div>
      </div>
    </div>
  </div>
<style>
    #topbar { height: 44px; display: flex; gap: 8px; align-items: center; padding: 0 12px; border-bottom: 1px solid var(--border); background: linear-gradient(180deg, #12151d, #0f121a); position: sticky; top: 0; z-index: 10; }
    button { padding: 6px 10px; border-radius: 8px; border: 1px solid var(--border); background: var(--btn); color: var(--text); cursor: pointer; }
    button:hover { background: var(--btnh); }
    .tag { padding: 3px 8px; border:1px solid var(--border); border-radius:999px; color: var(--muted); }
    #container { display: grid; grid-template-columns: 300px 1fr; height: calc(100% - 44px); }
    #sidebar { border-right: 1px solid var(--border); background: var(--panel); overflow: auto; display: flex; flex-direction: column; }
    #sidebar-top { padding: 8px; display: flex; gap: 6px; align-items: center; border-bottom: 1px solid var(--border); }
    #file-search { flex: 1 1 auto; background:#10131a; border:1px solid var(--border); border-radius:8px; padding:6px 8px; color: var(--text); }
    #files { padding: 10px; margin: 0; list-style: none; font-size: 13px; }
   
    .dim { color: var(--muted); }
    #main { display: grid; grid-template-rows: 1fr 260px; background: var(--bg2); }
    #editor-area { display: grid; grid-template-columns: 1fr 0; }
    #editor { height: 100%; }
    #preview { border-left: 1px solid var(--border); padding: 12px; overflow: auto; background:#0a0f17; }
    #bottom { border-top: 1px solid var(--border); background: #0c111b; display: grid; grid-template-rows: 36px 1fr; }
    #tabs { display: flex; gap: 6px; align-items: center; padding: 6px 8px; border-bottom: 1px solid var(--border); background: #0e1420; }
    .tab { padding:6px 10px; border:1px solid var(--border); border-radius:8px; background:#111724; cursor:pointer; }
    .tab.active { background:#172034; color: var(--accent); }
    #log, #termlog, #xterm { padding: 8px; overflow: auto; font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 12px; white-space: pre-wrap; }
    #log { display: block; }
    #termlog { display: none; }
    #xterm { display:none; height: 100%; padding: 0; }
    .status { margin-left: auto; color: var(--muted); }
    .hidden { display:none !important; }
   
   
</style>