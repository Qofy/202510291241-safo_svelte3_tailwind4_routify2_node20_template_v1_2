<script>
	import { onMount } from "svelte";
export let splitViewActive =false
export let editor = {}
export let editorSplit ={}
export let splitEditor ={}
  onMount(()=>{
    document.getElementById('split-view-toggle').onclick = () => {
  splitViewActive = !splitViewActive;
  const container = document.getElementById('editor-container');

  const divider = document.getElementById('editor-split-divider');

  if (splitViewActive) {
    container.style.gridTemplateColumns = '1fr 4px 1fr';
    splitEditor.style.display = 'block';
    divider.style.display = 'block';
     

    
    setTimeout(() => {
      if (editor) editor.layout();
      if (editorSplit) editorSplit.layout();
    }, 0);
  } else {
    container.style.gridTemplateColumns = '1fr 0';
    splitEditor.style.display = 'none';
    divider.style.display = 'none';
    setTimeout(() => editor && editor.layout(), 0);
  }
};

  const splitBtn = document.getElementById('split-view-toggle');

  })
</script>

  <button class="split-view-btn" id="split-view-toggle" title="Toggle split view">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <rect x="3" y="3" width="7" height="18" rx="1"/>
              <rect x="14" y="3" width="7" height="18" rx="1"/>
            </svg>
            Split
          </button>

<style>
      /* Split view button */
    .split-view-btn {
      padding: 6px 10px;
      border: 1px solid var(--border);
      border-radius: 6px;
      background: var(--btn);
      color: var(--text);
      cursor: pointer;
      transition: all 0.2s;
      display: flex;
      align-items: center;
      gap: 4px;
      margin-left: auto;
    }
    .split-view-btn:hover {
      background: var(--btnh);
      border-color: var(--accent);
    }
    .split-view-btn svg {
      width: 14px;
      height: 14px;
    }
</style>