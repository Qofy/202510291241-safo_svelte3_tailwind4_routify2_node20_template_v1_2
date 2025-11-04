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
      #sidebar-top { padding: 8px; display: flex; gap: 6px; align-items: center; border-bottom: 1px solid var(--border); }  
       #file-search { flex: 1 1 auto; background:#10131a; border:1px solid var(--border); border-radius:8px; padding:3px 4px; color: var(--text);font-size: .8rem; font-weight: 600;}
       #sidebar { border-right: 1px solid var(--border); background: var(--panel); overflow: auto; display: flex !important; flex-direction: column; }
       :global(#sidebar.hidden) { display: none !important; }

    #files { padding: 10px; margin: 0; list-style: none; font-size: 13px; }
  </style>