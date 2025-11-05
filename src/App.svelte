
<!-- src/App.svelte -->
<script>
  import { Router } from '@roxi/routify';
  import { routes } from '../.routify/routes';
  import Notification from './components/Notification.svelte';
</script>

  <Router {routes} />
  <Notification />
  
  