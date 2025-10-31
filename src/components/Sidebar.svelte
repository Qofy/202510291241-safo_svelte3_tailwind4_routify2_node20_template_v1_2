<script>
	import SearchButton from "./btn/SearchButton.svelte";


</script>


<div id="sidebar">
      <div id="sidebar-top">
        <input id="file-search" type="text" placeholder="Search files by name…" />
        <SearchButton/>
      </div>
      <ul id="files"></ul>
    </div>

  <style>
     #sidebar { border-right: 1px solid var(--border); background: var(--panel); overflow: auto; display: flex; flex-direction: column; }
    #sidebar-top { padding: 8px; display: flex; gap: 6px; align-items: center; border-bottom: 1px solid var(--border); }

    /* .sidebar { width: 260px; min-width: 180px; max-width: 480px; }
.sidebar.collapsed { width: 0; min-width: 0; }
.editor-wrap { flex: 1 1 auto; min-width: 0; display: flex; } */
  </style>