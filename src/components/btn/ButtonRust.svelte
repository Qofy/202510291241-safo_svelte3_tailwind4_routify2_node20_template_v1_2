<script>
	import { onMount } from "svelte";
export let ws = null
onMount(()=>{
  document.getElementById('btn-rust').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'rust' })); };
})
</script>
    <button id="btn-rust">Rust LSP</button>
