<script>
	import { onMount } from "svelte";

  onMount(()=>{
    document.getElementById('ai-save').onclick = () => {
  const cfg = {
    provider: document.getElementById('ai-provider').value,
    model: document.getElementById('ai-model').value.trim(),
    key: document.getElementById('ai-key').value.trim(),
  };
  localStorage.setItem('aiCfg', JSON.stringify(cfg));
  aiModal.classList.add('hidden');
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(JSON.stringify({ type: 'ai_set_config', config: cfg }));
  }
};
  })
</script>
      <button id="ai-save">Save</button>
