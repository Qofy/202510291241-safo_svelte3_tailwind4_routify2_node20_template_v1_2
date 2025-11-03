<script>
	import { onMount } from "svelte";

  function toast(message, duration = 3000) {
  const toastEl = document.createElement('div');
  toastEl.style.cssText = `
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: #2a3242;
    color: #d7dde8;
    padding: 12px 20px;
    border-radius: 8px;
    border: 1px solid #61dafb;
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
    z-index: 1000;
    font-size: 14px;
    animation: slideIn 0.3s ease;
  `;
  toastEl.textContent = message;
  document.body.appendChild(toastEl);

  setTimeout(() => {
    toastEl.style.animation = 'slideOut 0.3s ease';
    setTimeout(() => toastEl.remove(), 300);
  }, duration);
}

  onMount(()=>{
    document.getElementById('btn-search').onclick = async () => {
  const q = (document.getElementById('file-search').value || '').trim();
  if (!q) {
    toast('Please enter a search query', 2000);
    return;
  }

  setStatus('Searching...');
  const res = await fetch('http://127.0.0.1:8788/api/search?' + new URLSearchParams({ q, max: '500' }));
  const hits = await res.json();

  // Display results in search tab
  const searchResults = document.getElementById('search-results');
  searchResults.innerHTML = '';

  if (hits.length === 0) {
    searchResults.innerHTML = '<div style="padding: 16px; color: #9aa3b2;">No results found</div>';
    setStatus(`No results for "${q}"`);
    setTab('search');
    return;
  }

  // Group results by file
  const groupedByFile = {};
  for (const hit of hits) {
    if (!groupedByFile[hit.path]) {
      groupedByFile[hit.path] = [];
    }
    groupedByFile[hit.path].push(hit);
  }

  // Render grouped results
  for (const [filePath, fileHits] of Object.entries(groupedByFile)) {
    // File header
    const fileHeader = document.createElement('div');
    fileHeader.style.cssText = 'padding: 8px; background: #0e1420; border-bottom: 2px solid var(--border); font-weight: 600; color: var(--accent); position: sticky; top: 0;';
    fileHeader.textContent = `${filePath} (${fileHits.length} ${fileHits.length === 1 ? 'match' : 'matches'})`;
    searchResults.appendChild(fileHeader);

    // Results for this file
    for (const hit of fileHits) {
      const item = document.createElement('div');
      item.className = 'search-result-item';
      item.dataset.path = hit.path;
      item.dataset.line = hit.line;
      item.dataset.col = hit.col;

      // Location info
      const location = document.createElement('span');
      location.className = 'search-result-location';
      location.textContent = `Line ${hit.line}, Col ${hit.col}`;

      // Text with highlighted match
      const textDiv = document.createElement('div');
      textDiv.className = 'search-result-text';
      const highlightedText = hit.text.replace(
        new RegExp(q.replace(/[.*+?^${}()|[\\]\\\\]/g, '\\\\$&'), 'gi'),
        match => `<span class="search-result-match">${match}</span>`
      );
      textDiv.innerHTML = highlightedText;

      item.appendChild(location);
      item.appendChild(textDiv);

      // Click handler to open file
      item.onclick = () => {
        openFile(hit.path, { preserveCursor: false }).then(() => {
          if (editor) {
            editor.setPosition({ lineNumber: hit.line, column: hit.col });
            editor.revealLineInCenter(hit.line);
            editor.focus();
          }
        });
      };

      searchResults.appendChild(item);
    }
  }

  setStatus(`Found ${hits.length} results in ${Object.keys(groupedByFile).length} files`);
  setTab('search');
};
  })
</script>

<button id="btn-search" >Find in files</button>

<style>
  #btn-search{
    font-size: .8rem;
  }
</style>
