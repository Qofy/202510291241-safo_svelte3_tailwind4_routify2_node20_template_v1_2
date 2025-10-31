<script>
	import { onMount } from "svelte";
let aiModal
  onMount(()=>{
    aiModal = document.getElementById('ai-modal')
    
document.getElementById('ai-cancel').onclick = () => aiModal.classList.add('hidden');
  })
</script>

      <button id="ai-cancel">Cancel</button>
