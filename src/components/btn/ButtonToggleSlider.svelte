<script>
	import { onMount } from "svelte";
export let editor = {}
export let editorSplit ={}
export let splitViewActive = false
  onMount(()=>{
    document.getElementById('toggle-sidebar').onclick = () => {
  const sb = document.getElementById('sidebar');
  const container = document.getElementById('container');
  const divider = document.getElementById('sidebar-divider');

  if (sb.classList.contains('hidden')) {
    sb.classList.remove('hidden');
    divider.style.display = 'block';
    const savedWidth = localStorage.getItem('sidebarWidth') || '300px';
    container.style.gridTemplateColumns = `${savedWidth||"1fr"} 4px 1fr`;
  } else {
    sb.classList.add('hidden');
    divider.style.display = 'none';
    if (splitViewActive){
      container.style.gridTemplateColumns="100%"
    }else {
        sb.classList.add('hidden');
        divider.style.display = 'none';
        
        if (splitViewActive) {
          // container.style.gridTemplateColumns = "100vw";
          // Increase split editor sizes when sidebar is hidden
          container.style.gridTemplateColumns = '1fr 4px 1fr';
        } else {
          container.style.gridTemplateColumns = '100%';
        }
      }
  }

  

  setTimeout(() => {
    if (editor) editor.layout();
    if (splitViewActive && editorSplit) editorSplit.layout();
  }, 0);
};
  })
</script>
    <button id="toggle-sidebar">☰</button>
