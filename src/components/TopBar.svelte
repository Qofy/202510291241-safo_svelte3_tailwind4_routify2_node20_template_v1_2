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

  <style>
        #topbar { height: 44px; display: flex; gap: 8px; align-items: center; padding: 0 12px; border-bottom: 1px solid var(--border); background: linear-gradient(180deg, #12151d, #0f121a); position: sticky; top: 0; z-index: 10; }
 .status { margin-left: auto; color: var(--muted); }
    .dim { color: var(--muted); }
  </style>