<script>
	import { onMount } from "svelte";

  onMount(()=>{
    document.getElementById('btn-fmt').onclick = () => {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ type: 'run_once', cmd: 'cargo fmt' }));
    flashButton('btn-fmt'); // visual feedback
  }
};
  })
  
   function flashButton(id){
  const b = document.getElementById(id);
  b.classList.add('active');
  setTimeout(()=>b.classList.remove('active'), 120);
}

</script>
   <button id="btn-fmt" title="cargo fmt">FMT</button>