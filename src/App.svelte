
<!-- src/App.svelte -->
<script>
  import { Router } from '@roxi/routify';
  import { routes } from '../.routify/routes';
  import Notification from './components/Notification.svelte';
</script>

<div class="min-h-screen bg-black text-white p-6 flex h-full">
  <Router {routes} />
  <Notification />
</div>