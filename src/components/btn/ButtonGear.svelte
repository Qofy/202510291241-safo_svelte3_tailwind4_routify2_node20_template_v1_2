<script>
	import { onMount } from "svelte";
  let aiModal;

  onMount(()=>{
    aiModal = document.getElementById('ai-modal')
    document.getElementById('btn-gear').onclick = () => {
  const cfg = JSON.parse(localStorage.getItem('aiCfg') || '{"provider":"ollama","model":"llama3.1","key":""}');
  document.getElementById('ai-provider').value = cfg.provider;
  document.getElementById('ai-model').value = cfg.model || '';
  document.getElementById('ai-key').value = cfg.key || '';
  aiModal.classList.remove('hidden');
};
  })
</script>

    <button id="btn-gear">⚙︎</button>
