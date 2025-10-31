<script>
	import { onMount } from "svelte";

  onMount(()=>{
document.getElementById('ai-cancel').onclick = () => aiModal.classList.add('hidden');
  })
</script>

      <button id="ai-cancel">Cancel</button>
