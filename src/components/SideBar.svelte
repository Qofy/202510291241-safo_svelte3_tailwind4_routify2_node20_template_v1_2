<script>

</script>


 <div id="sidebar">
      <div id="sidebar-top">
        <input id="file-search" type="text" placeholder="Search files by name…" />
        <button id="btn-search">Find in files</button>
      </div>
      <ul id="files"></ul>
    </div>

    <style>
  #sidebar { border-right: 1px solid var(--border); background: var(--panel); overflow: auto; display: flex; flex-direction: column; }
    #sidebar-top { padding: 8px; display: flex; gap: 6px; align-items: center; border-bottom: 1px solid var(--border); }
    #file-search { flex: 1 1 auto; background:#10131a; border:1px solid var(--border); border-radius:8px; padding:4px 4px; color: var(--text); font-size: .8rem;}
    #files { padding: 10px; margin: 0; list-style: none; font-size: 8px; }
    #btn-search{
      font-size: .8rem;
      font-weight: 500;
    }
    </style>