<script>
	import { onMount } from "svelte";

  onMount(()=>{
    document.getElementById('runner-stop').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_stop' })); }
  })
</script>

          <button id="runner-stop">Stop</button>