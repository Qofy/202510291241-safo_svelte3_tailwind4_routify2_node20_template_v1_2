<script>
	import { onMount } from "svelte";

  onMount(()=>{
document.getElementById('btn-save').onclick = () => saveCurrentFile();
  })

  // Save current file
// Save current file
async function saveCurrentFile() {
  if (!currentPath) {
    toast('No file open', 2000);
    return;
  }

  const body = { path: currentPath, content: editor.getValue() };
  try {
    const res = await fetch('http://127.0.0.1:8788/api/save', {
      method: 'POST',
      headers: { 'Content-Type':'application/json' },
      body: JSON.stringify(body)
    });
    const js = await res.json();

    if (js.ok) {
      originalContent = editor.getValue(); // Update original content
      setStatus('Saved ✓');
      toast('File saved successfully');

      // Mark file as clean in tabs
      if (activeFile) {
        markFileClean(activeFile);
      }

      // Save cursor position
      const pos = editor.getPosition();
      if (pos) {
        localStorage.setItem('cursor:' + normalizePath(currentPath), JSON.stringify(pos));
      }
    } else {
      toast('Save failed: ' + js.error, 5000);
      log(logEl, 'save failed: ' + js.error);
    }
  } catch (e) {
    toast('Save error: ' + e.message, 5000);
    log(logEl, 'save error: ' + e);
  }
}
</script>

    <button id="btn-save">Save</button>
