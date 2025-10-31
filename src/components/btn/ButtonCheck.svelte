<script>
	import { onMount } from "svelte";
  export let ws = null
  onMount(()=>{
    document.getElementById('btn-check').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'cargo', sub: 'check' })); };
  })
</script>

    <button id="btn-check">cargo check</button>
