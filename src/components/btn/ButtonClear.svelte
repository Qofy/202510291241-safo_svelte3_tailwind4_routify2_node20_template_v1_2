<script>
	import { onMount } from "svelte";

  onMount(()=>{
    document.getElementById('btn-clear').onclick = () => {
  const logEl = document.getElementById('log');
  const termEl = document.getElementById('termlog');
  const xtermEl = document.getElementById('xterm');

  if (logEl && logEl.style.display !== 'none') logEl.textContent='';
  if (termEl && termEl.style.display !== 'none') termEl.textContent='';
  if (xtermEl && xtermEl.style.display !== 'none' && terminal) terminal.clear();
};
  })
</script>

    <button id="btn-clear">Clear</button>
