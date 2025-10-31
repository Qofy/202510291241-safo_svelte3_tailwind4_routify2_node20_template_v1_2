<script>
	import { onMount } from "svelte";

  onMount(()=>{
    document.getElementById('runner-restart').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_restart' })); }
  })
</script>

          <button id="runner-restart">Restart</button>