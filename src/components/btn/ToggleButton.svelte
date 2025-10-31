<script>
	import { onMount } from "svelte";

  onMount(()=>{
document.getElementById('toggle-bottom-panel').onclick = () => {
  const bottomPanel = document.getElementById('bottom');
  const toggleBtn = document.getElementById('toggle-bottom-panel');

  bottomPanel.classList.toggle('collapsed');
  toggleBtn.classList.toggle('collapsed');

  // Trigger Monaco editor resize
  setTimeout(() => {
    if (editor) editor.layout();
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