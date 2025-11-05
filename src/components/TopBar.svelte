<script>
	import { onMount } from "svelte";
let ws,currentPath = null;


onMount(()=>{
const previewEl = document.getElementById('preview');
  document.getElementById('btn-preview').onclick = () => togglePreview();

  document.getElementById('btn-save').onclick = async () => {
  if (!currentPath) return;
  const body = { path: currentPath, content: editor.getValue() };
  const res = await fetch('http://127.0.0.1:8788/api/save', { method: 'POST', headers: { 'Content-Type':'application/json' }, body: JSON.stringify(body) });
  const js = await res.json();
  if (!js.ok) log(logEl, 'save failed: ' + js.error);
};

document.getElementById('btn-rust').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'rust' })); };
document.getElementById('btn-ts').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'ts' })); };
document.getElementById('btn-check').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'cargo', sub: 'check' })); };

function togglePreview() {
  const isMd = currentPath && (currentPath.endsWith('.md') || currentPath.endsWith('.markdown'));
  if (!isMd) { previewEl.classList.add('hidden'); document.getElementById('editor-area').style.gridTemplateColumns = '1fr 0'; return; }
  if (previewEl.classList.contains('hidden')) {
    previewEl.classList.remove('hidden');
    document.getElementById('editor-area').style.gridTemplateColumns = '1fr 1fr';
    renderMarkdown();
  } else {
    previewEl.classList.add('hidden');
    document.getElementById('editor-area').style.gridTemplateColumns = '1fr 0';
  }
}
})
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
     .tag { padding: 3px 8px; border:1px solid var(--border); border-radius:999px; color: var(--muted); }
  </style>