<script>
	import { editor } from "monaco-editor";
	import { onMount } from "svelte";

  export let editorSplit = null;
  onMount(()=>{
document.getElementById('toggle-bottom-panel').onclick = () => {
  const bottomPanel = document.getElementById('bottom');
  const toggleBtn = document.getElementById('toggle-bottom-panel');

  bottomPanel.classList.toggle('collapsed');
  toggleBtn.classList.toggle('collapsed');

  // Trigger Monaco editor resize
  setTimeout(() => {
    if (editor && typeof editor.layout ==="function") editor.layout();
    if (editorSplit) editorSplit.layout();
  }, 300);
};
  })
</script>

 <button id="toggle-bottom-panel" class="toggle-bottom-btn" title="Toggle bottom panel">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
              <path d="M3 15h18M3 9h18"/>
              <path d="M8 15l-5 5-5-5" stroke-width="2.5"/>
            </svg>
          </button>
<style>
   /* Toggle bottom panel button with SVG */
    .toggle-bottom-btn {
      padding: 6px 10px;
      border: 1px solid var(--border);
      border-radius: 6px;
      background: var(--btn);
      color: var(--accent);
      cursor: pointer;
      transition: all 0.2s ease;
      display: flex;
      align-items: center;
      justify-content: center;
      margin-right: 4px;
    }
    .toggle-bottom-btn:hover {
      background: var(--btnh);
      border-color: var(--accent);
    }
    .toggle-bottom-btn svg {
      width: 16px;
      height: 16px;
      transition: transform 0.3s ease;
    }
   
</style>