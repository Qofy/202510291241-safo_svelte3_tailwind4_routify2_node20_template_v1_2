<script>
  import { onMount } from 'svelte';
  
  export let openFiles = [];
  export let currentFile = null;
  export let onFileSelect = () => {};
  export let onFileClose = () => {};
  
  function isModified(file) {
    return file.modified || false;
  }
</script>

<div class="file-tabs">
  {#each openFiles as file}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div 
      class="tab-item {currentFile === file.path ? 'active' : ''}"
      on:click={() => onFileSelect(file.path)}
    >
      <span class="tab-name">{file.name}</span>
      {#if isModified(file)}
        <span class="modified-indicator">●</span>
      {/if}
      <button 
        class="close-btn" 
        on:click|stopPropagation={() => onFileClose(file.path)}
      >
        ×
      </button>
    </div>
  {/each}
</div>

<style>
  .file-tabs {
    display: flex;
    gap: 2px;
    background: #0e1420;
    border-bottom: 1px solid var(--border);
    overflow-x: auto;
    padding: 4px;
  }
  
  .tab-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: #1a1f2e;
    border: 1px solid var(--border);
    border-radius: 6px 6px 0 0;
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.2s;
  }
  
  .tab-item:hover {
    background: #222938;
  }
  
  .tab-item.active {
    background: #2a3242;
    border-bottom-color: transparent;
  }
  
  .tab-name {
    font-size: 0.85rem;
    color: var(--text);
  }
  
  .modified-indicator {
    color: #fbbf24;
    font-size: .7rem;
    line-height: 1;
  }
  
  .close-btn {
    background: none;
    border: none;
    color: var(--muted);
    font-size: 1.3rem;
    line-height: 1;
    padding: 0;
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 3px;
  }
  
  .close-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    color: var(--text);
  }
</style>