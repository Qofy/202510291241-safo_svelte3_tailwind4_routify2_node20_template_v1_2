<script>
	import { onMount } from "svelte";
 export let currentPath = null;
 export let previewEl;
  onMount(()=>{
 previewEl = document.getElementById('preview');

    document.getElementById('btn-preview').onclick = () => togglePreview();
  })

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
};
</script>


    <button id="btn-preview">Preview</button>
