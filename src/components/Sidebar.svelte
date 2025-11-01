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
     #sidebar { border-right: 1px solid var(--border); background: var(--panel);  display: flex; flex-direction: column; }
    #sidebar-top { padding: 8px; display: flex; gap: 6px; align-items: center; border-bottom: 1px solid var(--border); }

    
  </style>