<script>
// src/components/Notification.svelte
  import { notification } from '../stores/notificationStore.js';
  let visible = true;
  $: msg = $notification;
</script>

{#if msg && visible}
	<div class="fixed top-0 left-0 right-0 text-black 
		{msg.type === 'success' ? 'bg-green-600 text-white' : ''}
		{msg.type === 'error' ? 'bg-red-600 text-white' : ''}
		{msg.type === 'info' ? 'bg-yellow-500 text-black' : ''}
		p-3 shadow-lg z-50 flex justify-between items-center">
    <div>{msg.message}</div>
    <button class="px-2 py-1 bg-black text-yellow-400 rounded hover:bg-gray-900" on:click={() => visible = false}>X CLOSE</button>
  </div>
{/if}

