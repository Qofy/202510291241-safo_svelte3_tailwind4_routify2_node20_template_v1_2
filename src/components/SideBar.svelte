<script>
	import { onMount } from "svelte";
  let filesEl;

  onMount(() => {
    filesEl = document.getElementById('files');
    const fileSearchInput = document.getElementById('file-search');
    
    // Real-time search as user types
    if (fileSearchInput) {
      fileSearchInput.addEventListener('input', (e) => {
        const searchTerm = e.target.value.toLowerCase().trim();
        filterFiles(searchTerm);
      });
    }

    // Toggle sidebar visibility
    const toggleBtn = document.getElementById('toggle-sidebar');
    if (toggleBtn) {
      toggleBtn.onclick = () => {
        const sb = document.getElementById('sidebar');
        const container = document.getElementById('container');
        if (sb && container) {
          if (sb.classList.contains('hidden')) { 
            sb.classList.remove('hidden'); 
            container.style.gridTemplateColumns = '300px 1fr'; 
          } else { 
            sb.classList.add('hidden'); 
            container.style.gridTemplateColumns = '0px 1fr'; 
          }
        }
      };
    }

    // Search in file contents
    const searchBtn = document.getElementById('btn-search');
    if (searchBtn) {
      searchBtn.onclick = async () => {
        const q = (document.getElementById('file-search').value || '').trim(); 
        if (!q) return;
        
        const logEl = document.getElementById('log');
        if (!logEl) {
          console.error('Log element not found');
          return;
        }

        try {
          const res = await fetch('http://127.0.0.1:8788/api/search?' + new URLSearchParams({ q, max: '200' }));
          
          if (!res.ok) {
            logEl.textContent += `[ERROR] Search failed: ${res.status}\n`;
            return;
          }

          const hits = await res.json();
          const out = hits.map(h => `${h.path}:${h.line}:${h.col}  ${h.text}`).join('\n');
          
          // Display in log panel
          logEl.textContent += `-- search: "${q}" --\n` + out + '\n-- end --\n';
          logEl.scrollTop = logEl.scrollHeight;
          
          // Auto-switch to LSP tab to show results
          const lspTab = document.querySelector('.tab[data-tab="lsp"]');
          if (lspTab) lspTab.click();
        } catch (error) {
          console.error('Search error:', error);
          logEl.textContent += `[ERROR] Search failed: ${error.message}\n`;
        }
      };
    }
  });

  function filterFiles(searchTerm) {
  if (!filesEl) return;
  
  const allItems = document.querySelectorAll('#files li');
  
  if (!searchTerm) {
    // Show all files and reset folder states
    allItems.forEach(li => {
      li.style.display = '';
    });
    return;
  }
  
  // First pass: hide everything
  allItems.forEach(li => {
    li.style.display = 'none';
  });
  
  // Second pass: show matches and their parents
  allItems.forEach(li => {
    const fileNameEl = li.querySelector(':scope > .node .fname');
    if (fileNameEl) {
      const fileName = fileNameEl.textContent.toLowerCase();
      
      // If this file/folder matches the search
      if (fileName.startsWith(searchTerm) || fileName.includes(searchTerm)) {
        // Show this item
        li.style.display = '';
        
        // Show and expand all parent folders
        let parent = li.parentElement.closest('li');
        while (parent) {
          parent.style.display = '';
          
          // Expand the folder to show the match
          const twisty = parent.querySelector(':scope > .node .twisty');
          const childContainer = parent.querySelector(':scope > div:not(.node)');
          if (twisty && childContainer) {
            twisty.textContent = '▼';
            childContainer.style.display = 'block';
          }
          
          parent = parent.parentElement.closest('li');
        }
        
        // If this is a folder that matches, show its direct children too
        const childContainer = li.querySelector(':scope > div:not(.node)');
        if (childContainer) {
          childContainer.style.display = 'block';
          const twisty = li.querySelector(':scope > .node .twisty');
          if (twisty) twisty.textContent = '▼';
        }
      }
    }
  });
}
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