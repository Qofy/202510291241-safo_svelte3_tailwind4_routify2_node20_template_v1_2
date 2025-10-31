<script>
	import { onMount } from "svelte";
  export let ws = null
  onMount(()=>{
    document.getElementById('btn-ts').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'ts' })); };
  })
</script>

    <button id="btn-ts">TS LSP</button>
