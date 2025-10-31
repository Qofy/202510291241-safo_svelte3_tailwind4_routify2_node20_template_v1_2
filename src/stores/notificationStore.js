// src/stores/notificationStore.js
import { writable } from 'svelte/store';

export const notification = (() => {

  const store = writable(null);

  function show(message, type = 'info') {
    store.set({ message, type, timestamp: Date.now() });
  }

  return {
    subscribe: store.subscribe,
    success: (msg) => show(msg, 'success'),
    error: (msg) => show(msg, 'error'),
    info: (msg) => show(msg, 'info')
  };
})();
