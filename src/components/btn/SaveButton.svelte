<script>
	import { onMount } from "svelte";

 export let currentPath = null;

 function toast(message, duration = 3000) {
  const toastEl = document.createElement('div');
  toastEl.style.cssText = `
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: #2a3242;
    color: #d7dde8;
    padding: 12px 20px;
    border-radius: 8px;
    border: 1px solid #61dafb;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    z-index: 1000;
    font-size: 14px;
    animation: slideIn 0.3s ease;
  `;
  toastEl.textContent = message;
  document.body.appendChild(toastEl);

  setTimeout(() => {
    toastEl.style.animation = 'slideOut 0.3s ease';
    setTimeout(() => toastEl.remove(), 300);
  }, duration);
}

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
