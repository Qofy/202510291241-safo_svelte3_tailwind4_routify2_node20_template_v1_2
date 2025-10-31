<script>
	import { onMount } from "svelte";
  let ws = null;
  onMount(()=>{
    document.getElementById('runner-start').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_start' })); }
  })
</script>

          <button id="runner-start">Start</button>
