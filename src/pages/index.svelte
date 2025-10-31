<script context="module">
	export const prerender = true;
	// export let scoped = "";
	// export let scopedSync = "";
</script>
<script>
 
  // import Hello from "../components/Hello.svelte";

  import {onMount} from "svelte";
  import * as monaco from 'monaco-editor';

	import TopBar from "../components/TopBar.svelte";
	import Sidebar from "../components/Sidebar.svelte";
	import Main from "../components/Main.svelte";
	import CommaondPalette from "../components/CommaondPalette.svelte";
	import AIProvider from "../components/AIProvider.svelte";
let editor, editorSplit, ws, currentPath = null;
let terminal, fitAddon, termStarted = false;
let originalContent = ''; // Track original content for dirty check

// DOM element references (initialized after DOM is ready)
let statusEl, wsPathEl, logEl, termEl, filesEl, xtermEl, previewEl, aiModal;

// File tabs state
const openFiles = new Map(); // path -> {content, cursorPosition, viewState}
let activeFile = null;
let splitViewActive = false;
let splitEditor ={}



  onMount(()=>{
    // notification.info("This is a notification")

        editor = monaco.editor.create(document.getElementById('editor'), {
       value: '// Select a file on the left to open it.',
    language: 'plaintext',
    automaticLayout: true,
    wordWrap: 'off',
    scrollBeyondLastColumn: 5,
    scrollbar: { horizontal: 'auto', vertical: 'auto', horizontalScrollbarSize: 12, verticalScrollbarSize: 12 },
    theme: 'vs-dark',
    minimap: {
      enabled: true,
      size: 'proportional',
      maxColumn: 120,
      renderCharacters: true,
      showSlider: 'always',
      scale: 2
    },
    fontLigatures: true,
    fontSize: 14,
      
    });

     splitEditor = document.getElementById('editor-split');
   const divider = document.getElementById('editor-split-divider');
      editorSplit = monaco.editor.create(splitEditor, {
        value: editor ? editor.getValue() : '',
        language: editor ? editor.getModel().getLanguageId() : 'plaintext',
        theme: 'vs-dark',
        automaticLayout: false,
      });

  //     window.require.config({ paths: { 'vs': '/vendor/monaco/vs' } });
  // window.require(['vs/editor/editor.main', 'vs/basic-languages/shell/shell'], function(monaco) {
    // Use monaco here
     // Initialize DOM element references
  statusEl = document.getElementById('status');
  wsPathEl = document.getElementById('ws-path');
  logEl = document.getElementById('log');
  termEl = document.getElementById('termlog');
  filesEl = document.getElementById('files');
  xtermEl = document.getElementById('xterm');
  previewEl = document.getElementById('preview');
  aiModal = document.getElementById('ai-modal');


  editor.onDidChangeModelContent(() => {
    // Mark file as dirty if content changed
    if (activeFile && originalContent !== editor.getValue()) {
      markFileDirty(activeFile);
    }

    // Render markdown preview if applicable
    if (currentPath && (currentPath.endsWith('.md') || currentPath.endsWith('.markdown')) && !previewEl.classList.contains('hidden')) {
      renderMarkdown();
    }
  });

  // Copy on keyboard selection with Shift held
  editor.onDidChangeCursorSelection(async (e) => {
    if (!shiftKeyHeld) return; // Only copy if shift is held during keyboard selection
    if (e.selection.isEmpty()) return;
    const model = editor.getModel();
    if (!model) return;
    const text = model.getValueInRange(e.selection);
    if (text && navigator.clipboard) {
      try {
        await navigator.clipboard.writeText(text);
        setStatus('copied selection');
      } catch {}
    }
  });

  setWorkspacePath();
  connectWS();
  listFiles();

  // Setup tab click handlers
  for (const t of document.querySelectorAll('.tab')) {
    t.onclick = () => setTab(t.dataset.tab);
  }

  // Setup resizable dividers
  setupResizableDivider('sidebar-divider', false);
  setupResizableDivider('bottom-divider', true);
  setupResizableDivider('editor-split-divider', false);

  // Restore previous session
  restoreSession();
  // });






// document.getElementById('btn-gear').onclick = () => {
//   const cfg = JSON.parse(localStorage.getItem('aiCfg') || '{"provider":"ollama","model":"llama3.1","key":""}');
//   document.getElementById('ai-provider').value = cfg.provider;
//   document.getElementById('ai-model').value = cfg.model || '';
//   document.getElementById('ai-key').value = cfg.key || '';
//   aiModal.classList.remove('hidden');
// };
// document.getElementById('ai-cancel').onclick = () => aiModal.classList.add('hidden');
// document.getElementById('ai-save').onclick = () => {
//   const cfg = {
//     provider: document.getElementById('ai-provider').value,
//     model: document.getElementById('ai-model').value.trim(),
//     key: document.getElementById('ai-key').value.trim(),
//   };
//   localStorage.setItem('aiCfg', JSON.stringify(cfg));
//   aiModal.classList.add('hidden');
//   if (ws && ws.readyState === WebSocket.OPEN) {
//     ws.send(JSON.stringify({ type: 'ai_set_config', config: cfg }));
//   }
// };


// ==================== SPLIT VIEW ====================
// document.getElementById('split-view-toggle').onclick = () => {
//   splitViewActive = !splitViewActive;
//   const container = document.getElementById('editor-container');
//   const splitEditor = document.getElementById('editor-split');
//   const divider = document.getElementById('editor-split-divider');

//   if (splitViewActive) {
//     container.style.gridTemplateColumns = '1fr 4px 1fr';
//     splitEditor.style.display = 'block';
//     divider.style.display = 'block';

//     if (!editorSplit && window.monaco) {
//       editorSplit = monaco.editor.create(splitEditor, {
//         value: editor ? editor.getValue() : '',
//         language: editor ? editor.getModel().getLanguageId() : 'plaintext',
//         theme: 'vs-dark',
//         automaticLayout: false,
//       });
//     }
//     setTimeout(() => {
//       if (editor) editor.layout();
//       if (editorSplit) editorSplit.layout();
//     }, 0);
//   } else {
//     container.style.gridTemplateColumns = '1fr 0';
//     splitEditor.style.display = 'none';
//     divider.style.display = 'none';
//     setTimeout(() => editor && editor.layout(), 0);
//   }
// };

  document.addEventListener('mousemove', (e) => {
    if (!isResizing) return;

    const delta = (isHorizontal ? e.clientY : e.clientX) - startPos;

    if (dividerId === 'sidebar-divider') {
      const newWidth = Math.max(180, Math.min(480, startSize + delta));
      document.getElementById('container').style.gridTemplateColumns = `${newWidth}px 4px 1fr`;
    } else if (dividerId === 'bottom-divider') {
      const newHeight = Math.max(100, Math.min(600, startSize - delta));
      document.getElementById('main').style.gridTemplateRows = `1fr ${newHeight}px`;
    } else if (dividerId === 'editor-split-divider') {
      const container = document.getElementById('editor-container');
      const totalWidth = container.offsetWidth;
      const newLeft = Math.max(200, Math.min(totalWidth - 200, startSize + delta));
      const rightWidth = totalWidth - newLeft - 4;
      container.style.gridTemplateColumns = `${newLeft}px 4px ${rightWidth}px`;
    }

    // Trigger editor layout
    setTimeout(() => {
      if (editor) editor.layout();
      if (editorSplit) editorSplit.layout();
    }, 0);
  });
  // ==================== TOGGLE BUTTONS ====================
// Toggle bottom panel collapse
// document.getElementById('toggle-bottom-panel').onclick = () => {
//   const bottomPanel = document.getElementById('bottom');
//   const toggleBtn = document.getElementById('toggle-bottom-panel');

//   bottomPanel.classList.toggle('collapsed');
//   toggleBtn.classList.toggle('collapsed');

//   // Trigger Monaco editor resize
//   setTimeout(() => {
//     if (editor) editor.layout();
//     if (editorSplit) editorSplit.layout();
//   }, 300);
// };
// Setup command palette input handler
document.addEventListener('DOMContentLoaded', () => {
  const input = document.getElementById('command-palette-input');
  const backdrop = document.getElementById('command-palette-backdrop');

  if (input) {
    input.addEventListener('input', (e) => {
      commandPaletteState.selectedIndex = 0;
      commandPaletteState.filteredCommands = searchCommands(e.target.value);
      renderCommandPalette(commandPaletteState.filteredCommands);
    });

    input.addEventListener('keydown', (e) => {
      const commands = commandPaletteState.filteredCommands;

      if (e.key === 'ArrowDown') {
        e.preventDefault();
        commandPaletteState.selectedIndex = Math.min(
          commandPaletteState.selectedIndex + 1,
          commands.length - 1
        );
        renderCommandPalette(commands);
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        commandPaletteState.selectedIndex = Math.max(
          commandPaletteState.selectedIndex - 1,
          0
        );
        renderCommandPalette(commands);
      } else if (e.key === 'Enter') {
        e.preventDefault();
        if (commands[commandPaletteState.selectedIndex]) {
          executeCommand(commands[commandPaletteState.selectedIndex]);
        }
      } else if (e.key === 'Escape') {
        e.preventDefault();
        hideCommandPalette();
      }
    });
  }

  if (backdrop) {
    backdrop.addEventListener('click', () => {
      hideCommandPalette();
    });
  }
});

// document.getElementById('btn-clear').onclick = () => {
//   const logEl = document.getElementById('log');
//   const termEl = document.getElementById('termlog');
//   const xtermEl = document.getElementById('xterm');

//   if (logEl && logEl.style.display !== 'none') logEl.textContent='';
//   if (termEl && termEl.style.display !== 'none') termEl.textContent='';
//   if (xtermEl && xtermEl.style.display !== 'none' && terminal) terminal.clear();
// };

// document.getElementById('runner-start').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_start' })); }
// document.getElementById('runner-restart').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_restart' })); }
// document.getElementById('runner-stop').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type:'runner_stop' })); }

// document.getElementById('toggle-sidebar').onclick = () => {
//   const sb = document.getElementById('sidebar');
//   const container = document.getElementById('container');
//   const divider = document.getElementById('sidebar-divider');

//   if (sb.classList.contains('hidden')) {
//     sb.classList.remove('hidden');
//     divider.style.display = 'block';
//     const savedWidth = localStorage.getItem('sidebarWidth') || '300px';
//     container.style.gridTemplateColumns = `${savedWidth} 4px 1fr`;
//   } else {
//     sb.classList.add('hidden');
//     divider.style.display = 'none';
//     container.style.gridTemplateColumns = '0px 0px 1fr';
//   }

//   setTimeout(() => {
//     if (editor) editor.layout();
//     if (editorSplit) editorSplit.layout();
//   }, 0);
// };

// document.getElementById('btn-preview').onclick = () => togglePreview();

// function togglePreview() {
//   const isMd = currentPath && (currentPath.endsWith('.md') || currentPath.endsWith('.markdown'));
//   if (!isMd) { previewEl.classList.add('hidden'); document.getElementById('editor-area').style.gridTemplateColumns = '1fr 0'; return; }
//   if (previewEl.classList.contains('hidden')) {
//     previewEl.classList.remove('hidden');
//     document.getElementById('editor-area').style.gridTemplateColumns = '1fr 1fr';
//     renderMarkdown();
//   } else {
//     previewEl.classList.add('hidden');
//     document.getElementById('editor-area').style.gridTemplateColumns = '1fr 0';
//   }
// };
// document.getElementById('btn-save').onclick = () => saveCurrentFile();

// document.getElementById('btn-rust').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'rust' })); };
// document.getElementById('btn-ts').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'lsp_spawn', lang: 'ts' })); };
// document.getElementById('btn-check').onclick = () => { if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'cargo', sub: 'check' })); };

// document.getElementById('btn-search').onclick = async () => {
//   const q = (document.getElementById('file-search').value || '').trim();
//   if (!q) {
//     toast('Please enter a search query', 2000);
//     return;
//   }

//   setStatus('Searching...');
//   const res = await fetch('http://127.0.0.1:8788/api/search?' + new URLSearchParams({ q, max: '500' }));
//   const hits = await res.json();

//   // Display results in search tab
//   const searchResults = document.getElementById('search-results');
//   searchResults.innerHTML = '';

//   if (hits.length === 0) {
//     searchResults.innerHTML = '<div style="padding: 16px; color: #9aa3b2;">No results found</div>';
//     setStatus(`No results for "${q}"`);
//     setTab('search');
//     return;
//   }

//   // Group results by file
//   const groupedByFile = {};
//   for (const hit of hits) {
//     if (!groupedByFile[hit.path]) {
//       groupedByFile[hit.path] = [];
//     }
//     groupedByFile[hit.path].push(hit);
//   }

//   // Render grouped results
//   for (const [filePath, fileHits] of Object.entries(groupedByFile)) {
//     // File header
//     const fileHeader = document.createElement('div');
//     fileHeader.style.cssText = 'padding: 8px; background: #0e1420; border-bottom: 2px solid var(--border); font-weight: 600; color: var(--accent); position: sticky; top: 0;';
//     fileHeader.textContent = `${filePath} (${fileHits.length} ${fileHits.length === 1 ? 'match' : 'matches'})`;
//     searchResults.appendChild(fileHeader);

//     // Results for this file
//     for (const hit of fileHits) {
//       const item = document.createElement('div');
//       item.className = 'search-result-item';
//       item.dataset.path = hit.path;
//       item.dataset.line = hit.line;
//       item.dataset.col = hit.col;

//       // Location info
//       const location = document.createElement('span');
//       location.className = 'search-result-location';
//       location.textContent = `Line ${hit.line}, Col ${hit.col}`;

//       // Text with highlighted match
//       const textDiv = document.createElement('div');
//       textDiv.className = 'search-result-text';
//       const highlightedText = hit.text.replace(
//         new RegExp(q.replace(/[.*+?^${}()|[\\]\\\\]/g, '\\\\$&'), 'gi'),
//         match => `<span class="search-result-match">${match}</span>`
//       );
//       textDiv.innerHTML = highlightedText;

//       item.appendChild(location);
//       item.appendChild(textDiv);

//       // Click handler to open file
//       item.onclick = () => {
//         openFile(hit.path, { preserveCursor: false }).then(() => {
//           if (editor) {
//             editor.setPosition({ lineNumber: hit.line, column: hit.col });
//             editor.revealLineInCenter(hit.line);
//             editor.focus();
//           }
//         });
//       };

//       searchResults.appendChild(item);
//     }
//   }

//   setStatus(`Found ${hits.length} results in ${Object.keys(groupedByFile).length} files`);
//   setTab('search');
// };
  });

  let isResizing=false;
  let dividerId, startPos,startSize,isHorizontal;
  //-------------------- closing mount-------------------------
//   function flashButton(id){
//   const b = document.getElementById(id);
//   b.classList.add('active');
//   setTimeout(()=>b.classList.remove('active'), 120);
// }



// ==================== FILE TABS MANAGEMENT ====================
function renderFileTabs() {
  const tabsContainer = document.getElementById('editor-tabs');
  const existingTabs = tabsContainer.querySelectorAll('.file-tab');
  existingTabs.forEach(tab => tab.remove());

  // const splitBtn = document.getElementById('split-view-toggle');


  openFiles.forEach((fileData, path) => {
    const tab = document.createElement('div');
    tab.className = 'file-tab';
    if (path === activeFile) tab.classList.add('active');
    if (fileData.dirty) tab.classList.add('dirty');

    const indicator = document.createElement('div');
    indicator.className = 'save-indicator';

    const fileName = path.split('/').pop();
    const name = document.createElement('span');
    name.textContent = fileName;

    const closeBtn = document.createElement('span');
    closeBtn.className = 'close-btn';
    closeBtn.textContent = '✕';
    closeBtn.onclick = (e) => {
      e.stopPropagation();
      closeFileTab(path);
    };

    tab.appendChild(indicator);
    tab.appendChild(name);
    tab.appendChild(closeBtn);
    tab.onclick = () => switchToFile(path);

    tabsContainer.insertBefore(tab, splitBtn);
  });

  saveSession();
}

function addFileTab(path) {
  if (!openFiles.has(path)) {
    openFiles.set(path, {
      content: editor ? editor.getValue() : '',
      cursorPosition: editor ? editor.getPosition() : null,
      viewState: editor ? editor.saveViewState() : null,
      dirty: false
    });
  }
  activeFile = path;
  renderFileTabs();
}

function closeFileTab(path) {
  if (openFiles.size <= 1) {
    // Don't close last tab
    return;
  }

  openFiles.delete(path);

  if (activeFile === path) {
    // Switch to another tab
    const remaining = Array.from(openFiles.keys());
    if (remaining.length > 0) {
      switchToFile(remaining[0]);
    }
  }

  renderFileTabs();
}

function switchToFile(path) {
  if (!openFiles.has(path)) return;

  // Save current editor state
  if (activeFile && editor) {
    const currentData = openFiles.get(activeFile);
    if (currentData) {
      currentData.content = editor.getValue();
      currentData.cursorPosition = editor.getPosition();
      currentData.viewState = editor.saveViewState();
      currentData.dirty = editor.getValue() !== originalContent;
    }
  }

  // Load new file
  activeFile = path;
  const fileData = openFiles.get(path);

  if (editor) {
    editor.setValue(fileData.content);
    originalContent = fileData.content;

    if (fileData.viewState) {
      editor.restoreViewState(fileData.viewState);
    } else if (fileData.cursorPosition) {
      editor.setPosition(fileData.cursorPosition);
      editor.revealLineInCenter(fileData.cursorPosition.lineNumber);
    }

    // Detect language
    const ext = path.split('.').pop();
    const langMap = { rs: 'rust', ts: 'typescript', js: 'javascript', json: 'json', html: 'html', css: 'css', md: 'markdown', toml: 'toml' };
    const lang = langMap[ext] || 'plaintext';
    monaco.editor.setModelLanguage(editor.getModel(), lang);
  }

  currentPath = path;
  renderFileTabs();
}

function markFileDirty(path) {
  const fileData = openFiles.get(path);
  if (fileData) {
    fileData.dirty = true;
    renderFileTabs();
  }
}

function markFileClean(path) {
  const fileData = openFiles.get(path);
  if (fileData) {
    fileData.dirty = false;
    fileData.content = editor ? editor.getValue() : fileData.content;
    renderFileTabs();
  }
}


// ==================== RESIZABLE DIVIDERS ====================
function setupResizableDivider(dividerId, isHorizontal) {
  const divider = document.getElementById(dividerId);
  let isResizing = false;
  let startPos = 0;
  let startSize = 0;

  divider.addEventListener('mousedown', (e) => {
    isResizing = true;
    startPos = isHorizontal ? e.clientY : e.clientX;
    divider.classList.add('dragging');

    if (dividerId === 'sidebar-divider') {
      const sidebar = document.getElementById('sidebar');
      startSize = sidebar.offsetWidth;
    } else if (dividerId === 'bottom-divider') {
      const bottom = document.getElementById('bottom');
      startSize = bottom.offsetHeight;
    } else if (dividerId === 'editor-split-divider') {
      const editorEl = document.getElementById('editor');
      startSize = editorEl.offsetWidth;
    }

    e.preventDefault();
  });



  document.addEventListener('mouseup', () => {
    if (isResizing) {
      isResizing = false;
      divider.classList.remove('dragging');
      saveSession();
    }
  });
}

// ==================== SESSION PERSISTENCE ====================
function saveSession() {
  const session = {
    openFiles: Array.from(openFiles.entries()),
    activeFile,
    splitViewActive,
    sidebarWidth: document.getElementById('sidebar').offsetWidth,
    bottomHeight: document.getElementById('bottom').offsetHeight,
  };
  localStorage.setItem('editorSession', JSON.stringify(session));
}

function restoreSession() {
  const sessionData = localStorage.getItem('editorSession');
  if (!sessionData) return;

  try {
    const session = JSON.parse(sessionData);

    // Restore open files
    if (session.openFiles) {
      session.openFiles.forEach(([path, data]) => {
        openFiles.set(path, data);
      });
    }

    // Restore active file
    if (session.activeFile && openFiles.has(session.activeFile)) {
      activeFile = session.activeFile;
      switchToFile(activeFile);
    }

    // Restore panel sizes
    if (session.sidebarWidth) {
      document.getElementById('container').style.gridTemplateColumns = `${session.sidebarWidth}px 4px 1fr`;
    }
    if (session.bottomHeight) {
      document.getElementById('main').style.gridTemplateRows = `1fr ${session.bottomHeight}px`;
    }

    renderFileTabs();
  } catch (e) {
    console.error('Failed to restore session:', e);
  }
}




// function log(el, s) { el.textContent += s + "\\n"; el.scrollTop = el.scrollHeight; }
function setStatus(s) { statusEl.textContent = s; }
function setWorkspacePath() { wsPathEl.textContent = location.host; }

// Check if current file has unsaved changes
function isDirty() {
  if (!editor || !currentPath) return false;
  return editor.getValue() !== originalContent;
}

// Show a temporary toast notification
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

// Append LSP message to LSP panel
function appendToLspPanel(msg) {
  const logDiv = document.getElementById('log');
  if (!logDiv) return;

  const line = document.createElement('div');
  line.style.cssText = 'padding: 4px 8px; border-bottom: 1px solid #1a1f2e; font-family: monospace; font-size: 12px;';

  try {
    const parsed = JSON.parse(msg);
    if (parsed.method) {
      line.innerHTML = `<span style="color: #61dafb">${parsed.method}</span>`;
    } else {
      line.textContent = msg.slice(0, 200);
    }
  } catch {
    line.textContent = msg.slice(0, 200);
  }

  logDiv.appendChild(line);
  logDiv.scrollTop = logDiv.scrollHeight;

  // Keep max 100 lines
  while (logDiv.children.length > 100) {
    logDiv.removeChild(logDiv.firstChild);
  }
}

// Append runner output to runner panel
function appendToRunnerPanel(msg) {
  const termlogDiv = document.getElementById('termlog');
  if (!termlogDiv) return;

  const line = document.createElement('div');
  line.style.cssText = 'padding: 2px 8px; font-family: monospace; font-size: 12px; white-space: pre-wrap;';
  line.textContent = msg;

  termlogDiv.appendChild(line);
  termlogDiv.scrollTop = termlogDiv.scrollHeight;

  // Keep max 500 lines
  while (termlogDiv.children.length > 500) {
    termlogDiv.removeChild(termlogDiv.firstChild);
  }
}

// // Save current file
// async function saveCurrentFile() {
//   if (!currentPath) {
//     toast('No file open', 2000);
//     return;
//   }

//   const body = { path: currentPath, content: editor.getValue() };
//   try {
//     const res = await fetch('http://127.0.0.1:8788/api/save', {
//       method: 'POST',
//       headers: { 'Content-Type':'application/json' },
//       body: JSON.stringify(body)
//     });
//     const js = await res.json();

//     if (js.ok) {
//       originalContent = editor.getValue(); // Update original content
//       setStatus('Saved ✓');
//       toast('File saved successfully');

//       // Mark file as clean in tabs
//       if (activeFile) {
//         markFileClean(activeFile);
//       }

//       // Save cursor position
//       const pos = editor.getPosition();
//       if (pos) {
//         localStorage.setItem('cursor:' + normalizePath(currentPath), JSON.stringify(pos));
//       }
//     } else {
//       toast('Save failed: ' + js.error, 5000);
//       log(logEl, 'save failed: ' + js.error);
//     }
//   } catch (e) {
//     toast('Save error: ' + e.message, 5000);
//     log(logEl, 'save error: ' + e);
//   }
// }

// ============================================================================
// COMMAND PALETTE
// ============================================================================

const commandRegistry = {
  // File operations
  'Save File': {
    category: 'File',
    icon: '💾',
    description: 'Save the current file',
    keybinding: 'Ctrl+S',
    action: () => saveCurrentFile()
  },
  'Close File': {
    category: 'File',
    icon: '✕',
    description: 'Close the current file',
    keybinding: 'Ctrl+W',
    action: () => { currentPath = null; editor.setValue('// No file open'); }
  },

  // Edit operations
  'Toggle Comment': {
    category: 'Edit',
    icon: '💬',
    description: 'Toggle line/block comment',
    keybinding: 'Ctrl+/',
    action: () => editor.trigger('keyboard', 'editor.action.commentLine', {})
  },
  'Format Document': {
    category: 'Edit',
    icon: '✨',
    description: 'Format the current document',
    keybinding: 'Shift+Alt+F',
    action: () => editor.trigger('keyboard', 'editor.action.formatDocument', {})
  },
  'Delete Line': {
    category: 'Edit',
    icon: '🗑',
    description: 'Delete the current line',
    keybinding: 'Ctrl+Shift+K',
    action: () => editor.trigger('keyboard', 'editor.action.deleteLines', {})
  },
  'Copy Line Down': {
    category: 'Edit',
    icon: '📋',
    description: 'Copy line down',
    keybinding: 'Shift+Alt+↓',
    action: () => editor.trigger('keyboard', 'editor.action.copyLinesDownAction', {})
  },
  'Move Line Up': {
    category: 'Edit',
    icon: '⬆',
    description: 'Move line up',
    keybinding: 'Alt+↑',
    action: () => editor.trigger('keyboard', 'editor.action.moveLinesUpAction', {})
  },
  'Move Line Down': {
    category: 'Edit',
    icon: '⬇',
    description: 'Move line down',
    keybinding: 'Alt+↓',
    action: () => editor.trigger('keyboard', 'editor.action.moveLinesDownAction', {})
  },

  // Selection
  'Select All Occurrences': {
    category: 'Selection',
    icon: '🔍',
    description: 'Select all occurrences of current selection',
    keybinding: 'Ctrl+Shift+L',
    action: () => editor.trigger('keyboard', 'editor.action.selectHighlights', {})
  },
  'Add Cursor Above': {
    category: 'Selection',
    icon: '⬆',
    description: 'Add cursor to line above',
    keybinding: 'Ctrl+Alt+↑',
    action: () => editor.trigger('keyboard', 'editor.action.insertCursorAbove', {})
  },
  'Add Cursor Below': {
    category: 'Selection',
    icon: '⬇',
    description: 'Add cursor to line below',
    keybinding: 'Ctrl+Alt+↓',
    action: () => editor.trigger('keyboard', 'editor.action.insertCursorBelow', {})
  },

  // View
  'Toggle Sidebar': {
    category: 'View',
    icon: '📁',
    description: 'Show/hide the file explorer',
    keybinding: 'Ctrl+B',
    action: () => document.getElementById('toggle-sidebar').click()
  },
  'Toggle Terminal': {
    category: 'View',
    icon: '⌨',
    description: 'Show/hide terminal panel',
    keybinding: 'Ctrl+`',
    action: () => setTab('terminal')
  },
  'Toggle Preview': {
    category: 'View',
    icon: '👁',
    description: 'Toggle markdown preview',
    keybinding: null,
    action: () => document.getElementById('btn-preview').click()
  },
  'Clear Terminal': {
    category: 'View',
    icon: '🧹',
    description: 'Clear terminal output',
    keybinding: null,
    action: () => document.getElementById('btn-clear').click()
  },

  // LSP
  'Start Rust LSP': {
    category: 'LSP',
    icon: '🦀',
    description: 'Start Rust language server',
    keybinding: null,
    action: () => document.getElementById('btn-rust').click()
  },
  'Start TypeScript LSP': {
    category: 'LSP',
    icon: '📘',
    description: 'Start TypeScript language server',
    keybinding: null,
    action: () => document.getElementById('btn-ts').click()
  },
  'Go to Definition': {
    category: 'LSP',
    icon: '➡',
    description: 'Jump to symbol definition',
    keybinding: 'F12',
    action: () => editor.trigger('keyboard', 'editor.action.revealDefinition', {})
  },
  'Find All References': {
    category: 'LSP',
    icon: '🔗',
    description: 'Find all references to symbol',
    keybinding: 'Shift+F12',
    action: () => editor.trigger('keyboard', 'editor.action.goToReferences', {})
  },
  'Rename Symbol': {
    category: 'LSP',
    icon: '✏',
    description: 'Rename symbol across files',
    keybinding: 'F2',
    action: () => editor.trigger('keyboard', 'editor.action.rename', {})
  },
  'Show Hover': {
    category: 'LSP',
    icon: 'ℹ',
    description: 'Show hover information',
    keybinding: 'Ctrl+K Ctrl+I',
    action: () => editor.trigger('keyboard', 'editor.action.showHover', {})
  },

  // Build
  'Run Cargo Check': {
    category: 'Build',
    icon: '✓',
    description: 'Run cargo check',
    keybinding: null,
    action: () => document.getElementById('btn-check').click()
  },
  'Run Cargo Fmt': {
    category: 'Build',
    icon: '✨',
    description: 'Format code with cargo fmt',
    keybinding: null,
    action: () => document.getElementById('btn-fmt').click()
  },

  // Help
  'Show Command Palette': {
    category: 'Help',
    icon: '⌘',
    description: 'Show all commands',
    keybinding: 'Ctrl+Shift+P',
    action: () => showCommandPalette()
  },
};

let commandPaletteState = {
  isOpen: false,
  selectedIndex: 0,
  filteredCommands: [],
};

// Fuzzy search implementation
function fuzzyMatch(text, query) {
  const textLower = text.toLowerCase();
  const queryLower = query.toLowerCase();

  let score = 0;
  let textIndex = 0;
  let queryIndex = 0;
  let lastMatchIndex = -1;

  while (queryIndex < queryLower.length && textIndex < textLower.length) {
    if (textLower[textIndex] === queryLower[queryIndex]) {
      // Bonus for consecutive matches
      if (lastMatchIndex === textIndex - 1) {
        score += 5;
      }
      // Bonus for match at start
      if (textIndex === 0) {
        score += 10;
      }
      // Bonus for match after separator
      if (textIndex > 0 && (text[textIndex - 1] === ' ' || text[textIndex - 1] === '-')) {
        score += 8;
      }
      score += 1;
      lastMatchIndex = textIndex;
      queryIndex++;
    } else {
      score -= 1;
    }
    textIndex++;
  }

  // Return null if not all query characters found
  if (queryIndex < queryLower.length) {
    return null;
  }

  return score;
}

function searchCommands(query) {
  if (!query.trim()) {
    // Return all commands grouped by category
    return Object.entries(commandRegistry).map(([name, cmd]) => ({
      name,
      ...cmd,
      score: 0
    }));
  }

  const results = [];
  for (const [name, cmd] of Object.entries(commandRegistry)) {
    const nameScore = fuzzyMatch(name, query);
    const descScore = cmd.description ? fuzzyMatch(cmd.description, query) * 0.5 : 0;
    const categoryScore = cmd.category ? fuzzyMatch(cmd.category, query) * 0.3 : 0;

    const totalScore = Math.max(nameScore || 0, descScore, categoryScore);

    if (totalScore > 0) {
      results.push({
        name,
        ...cmd,
        score: totalScore
      });
    }
  }

  return results.sort((a, b) => b.score - a.score);
}

function renderCommandPalette(commands) {
  const resultsEl = document.getElementById('command-palette-results');

  if (commands.length === 0) {
    resultsEl.innerHTML = `
      <div class="cmd-no-results">
        <div class="cmd-no-results-icon">🔍</div>
        <div class="cmd-no-results-text">No commands found</div>
      </div>
    `;
    return;
  }

  // Group by category
  const grouped = commands.reduce((acc, cmd) => {
    const cat = cmd.category || 'Other';
    if (!acc[cat]) acc[cat] = [];
    acc[cat].push(cmd);
    return acc;
  }, {});

  let html = '';
  const categories = Object.keys(grouped).sort();

  for (const category of categories) {
    html += `<div class="cmd-category-header">${category}</div>`;
    for (const cmd of grouped[category]) {
      const index = commands.indexOf(cmd);
      const selected = index === commandPaletteState.selectedIndex ? 'selected' : '';
      const keybinding = cmd.keybinding ? `
        <div class="cmd-keybinding">
          ${cmd.keybinding.split('+').map(k => `<span class="cmd-key">${k}</span>`).join('')}
        </div>
      ` : '';

      html += `
        <div class="cmd-item ${selected}" data-index="${index}">
          <div class="cmd-item-left">
            <div class="cmd-icon">${cmd.icon}</div>
            <div class="cmd-content">
              <div class="cmd-name">${cmd.name}</div>
              <div class="cmd-description">${cmd.description}</div>
            </div>
          </div>
          ${keybinding}
        </div>
      `;
    }
  }

  resultsEl.innerHTML = html;

  // Add click handlers
  resultsEl.querySelectorAll('.cmd-item').forEach(el => {
    el.addEventListener('click', () => {
      const index = parseInt(el.dataset.index);
      executeCommand(commands[index]);
    });
  });

  // Scroll selected into view
  const selectedEl = resultsEl.querySelector('.cmd-item.selected');
  if (selectedEl) {
    selectedEl.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
  }
}

function showCommandPalette() {
  const palette = document.getElementById('command-palette');
  const input = document.getElementById('command-palette-input');

  palette.classList.remove('hidden');
  input.value = '';
  input.focus();

  commandPaletteState.isOpen = true;
  commandPaletteState.selectedIndex = 0;
  commandPaletteState.filteredCommands = searchCommands('');

  renderCommandPalette(commandPaletteState.filteredCommands);
}

function hideCommandPalette() {
  const palette = document.getElementById('command-palette');
  palette.classList.add('hidden');
  commandPaletteState.isOpen = false;

  // Return focus to editor
  if (editor) {
    editor.focus();
  }
}

function executeCommand(cmd) {
  hideCommandPalette();
  try {
    cmd.action();
    toast(`Executed: ${cmd.name}`);
  } catch (e) {
    toast(`Error executing command: ${e.message}`, 5000);
  }
}



function renderProblems(){
  const root = document.getElementById('panel-problems');
  root.innerHTML = '';
  const list = document.createElement('div');
  problemsByFile.forEach((items, path) => {
    items.forEach(it => {
      const row = document.createElement('div');
      row.className = 'problem-row';
      const sev = (it.severity===1||it.severity===monaco.MarkerSeverity?.Error)?'E':'W';
      row.innerHTML = `
        <span class="badge ${sev==='E'?'err':'warn'}">${sev}</span>
        <span class="file">${path}</span>:
        <span class="line">${it.range.startLineNumber}</span>
        <span class="msg">${escapeHtml(shorten(it.message, 160))}</span>
      `;
      row.onclick = () => {
        openFile(path).then(() => {
          editor.revealLineInCenter(it.range.startLineNumber);
          editor.setPosition({ lineNumber: it.range.startLineNumber, column: it.range.startColumn || 1 });
          editor.focus();
        });
      };
      list.appendChild(row);
    });
  });
  root.appendChild(list);
}

function escapeHtml(s){ return s.replace(/[&<>"']/g, c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));}
function shorten(s,n){ return s.length>n? s.slice(0,n-1)+'…' : s; }
function applyMonacoMarkers(path, items){
  if (!editor || !currentPath) return;
  if (normalizePath(path) !== normalizePath(currentPath)) return;

  const markers = items.map(it => ({
    ...it.range,
    message: it.message,
    severity: (monaco.MarkerSeverity && (it.severity===1?monaco.MarkerSeverity.Error:monaco.MarkerSeverity.Warning)) || 8,
    source: it.source || 'lsp',
    code: it.code
  }));
  monaco.editor.setModelMarkers(editor.getModel(), 'lsp', markers);
}

function normalizePath(p){ return p.replace(/\\/g,'/'); }

function connectWS() {
  ws = new WebSocket(`ws://127.0.0.1:8789/ws`);
  ws.onopen = () => setStatus('WS connected');
  ws.onmessage = (ev) => {
    try {
      const msg = JSON.parse(ev.data);
      if (msg.type === 'info') setStatus(msg.message);
      if (msg.type === 'error') log(logEl, '[error] ' + msg.message);
      if (msg.type === 'lsp_recv') log(logEl, '[lsp] ' + msg.payload.slice(0, 500));
      if (msg.type === 'cargo_log') log(logEl, msg.chunk);
      if (msg.type === 'lsp_recv') {
        appendToLspPanel(msg.payload);
        handleLspMessagePayload(msg.payload);
      }

      if (msg.type === 'cargo_exit') log(logEl, 'cargo exit code ' + msg.code);
      //if (msg.type === 'runner_log') log(termEl, msg.chunk);
      if (msg.type === 'runner_log') {
        log(termEl, msg.chunk);               // stdout
      } else if (msg.type === 'runner_err') { // if you add a separate variant
        log(termEl, msg.chunk, { color: 'red' });
      }
      if (msg.type === 'runner_exit') log(termEl, 'runner exit code ' + msg.code);
      if (msg.type === 'terminal_data') { if (terminal) terminal.write(msg.chunk); }
      if (msg.type === 'ai_response') {
        if (!terminal) setTab('terminal'), ensureTerminalStarted();
        terminal.write('\r\n\x1b[35m🤖 ' + (msg.provider||'ai') + ':' + (msg.model||'') + '\x1b[0m\r\n');
        terminal.write((msg.response || '').replace(/\n/g, '\r\n') + '\r\n');
      }
      if (msg.type === 'run_once_output') {
            appendToRunnerPanel(msg.chunk); // or a toast/status
        } else if (msg.type === 'run_once_exit') {
            toast(`fmt exit ${msg.code}`);
            // Ask server to stat current file or just re-open blindly if not dirty
            if (currentPath && !isDirty()) {
            openFile(currentPath, { preserveCursor: true }); // implement to remember cursor
            }
        } else if (msg.type === 'file_changed') {
            // Server-side watcher notification
            if (currentPath && normalizePath(currentPath) === normalizePath(msg.path)) {
            if (!isDirty()) openFile(currentPath, { preserveCursor: true });
            else showReloadPrompt(msg.path);
            }
        }
        } catch (e) { log(logEl, 'bad message: ' + e); }
  };
  ws.onclose = () => setStatus('WS closed');
}

function normalizeChunk(s) {
  if (typeof s !== 'string') s = String(s);
  // 1) normalize real CRLF/CR to LF
  s = s.replace(/\r\n/g, '\n').replace(/\r/g, '\n');
  // 2) if server/lib left literal backslash+n, convert that too
  s = s.replace(/\\r\\n/g, '\n').replace(/\\n/g, '\n').replace(/\\r/g, '');
  return s;
}

function log(el, s, opts = {}) {
  const txt = normalizeChunk(s);
  if (opts.color === 'red') {
    const span = document.createElement('span');
    span.style.color = '#f87171';
    span.textContent = txt.endsWith('\n') ? txt : (txt + '\n');
    el.appendChild(span);
  } else {
    el.textContent += txt;
    if (!txt.endsWith('\n')) el.textContent += '\n';
  }
  el.scrollTop = el.scrollHeight;
}

const problemsByFile = new Map(); // path -> [{range,severity,msg,source,code}]
function handleLspMessagePayload(payloadStr) {
  try {
    const msg = JSON.parse(payloadStr);
    if (msg.method === 'textDocument/publishDiagnostics' && msg.params) {
      const { uri, diagnostics } = msg.params;
      // Convert file URI to workspace-relative path
      const path = uriToPath(uri); // implement below
      const items = (diagnostics || []).map(d => ({
        range: {
          startLineNumber: (d.range?.start?.line ?? 0) + 1,
          startColumn: (d.range?.start?.character ?? 0) + 1,
          endLineNumber: (d.range?.end?.line ?? 0) + 1,
          endColumn: (d.range?.end?.character ?? 0) + 1,
        },
        severity: monaco.Severity?.Error ? d.severity : d.severity, // raw; mapped later
        message: d.message || '',
        source: d.source || '',
        code: (typeof d.code === 'object' ? d.code?.value : d.code) || ''
      }));
      problemsByFile.set(path, items);
      renderProblems();                   // update Problems panel
      applyMonacoMarkers(path, items);    // set markers in editor if this file is open
    }
  } catch { /* ignore non-JSON logs */ }
}

function uriToPath(u){
  try {
    if (u.startsWith('file://')) {
      const url = new URL(u);
      return decodeURIComponent(url.pathname.replace(/^\/+/, '')); // remove leading slash on Windows if you want
    }
  } catch {}
  return u;
}

function maybeLayoutEditorSoon(){ setTimeout(()=> editor && editor.layout(), 0); }
// document.getElementById('toggle-sidebar').onclick = () => {
//   document.querySelector('.sidebar').classList.toggle('collapsed');
//   maybeLayoutEditorSoon();
// };
window.addEventListener('resize', maybeLayoutEditorSoon);
function showReloadPrompt(path){
  const ok = confirm(`${path} changed on disk. Reload and lose unsaved edits?`);
  if (ok) openFile(path, { preserveCursor:false });
}

function setTab(tab) {
  // Update all tab active states
  for (const t of document.querySelectorAll('.tab')) {
    t.classList.toggle('active', t.dataset.tab === tab);
  }

  // Update all panel active states
  const allTabs = ['lsp', 'runner', 'terminal', 'search', 'problems', 'log', 'xterm', 'termlog'];
  for (const id of allTabs) {
    const panel = document.getElementById('panel-' + id);
    const tabEl = document.getElementById('tab-' + id);
    if (panel) panel.classList.toggle('active', id === tab);
    if (tabEl) tabEl.classList.toggle('active', id === tab);
  }

  // Show/hide specific content elements based on tab
  const logEl = document.getElementById('log');
  const xtermEl = document.getElementById('xterm');
  const termEl = document.getElementById('termlog');
  const problemsEl = document.getElementById('problems');
  const searchEl = document.getElementById('search-results');

  if (logEl) logEl.style.display = (tab === 'lsp') ? 'block' : 'none';
  if (xtermEl) xtermEl.style.display = (tab === 'terminal' || tab === 'xterm') ? 'block' : 'none';
  if (termEl) termEl.style.display = (tab === 'runner') ? 'block' : 'none';
  if (problemsEl) problemsEl.style.display = (tab === 'problems') ? 'block' : 'none';
  if (searchEl) searchEl.style.display = (tab === 'search') ? 'block' : 'none';

  // Initialize terminal if switching to terminal tab
  if (tab === 'terminal' || tab === 'xterm') ensureTerminalStarted();

  // Trigger editor layout
  if (['terminal', 'xterm', 'problems'].includes(tab)) maybeLayoutEditorSoon();
}


function renderMarkdown() {
  if (!currentPath) return;
  const text = editor.getValue();
  const html = DOMPurify.sanitize(marked.parse(text));
  previewEl.innerHTML = html;
}

async function listFiles() {
  let res = {}
  try{
   res = await fetch('http://127.0.0.1:8789/api/list');
  }catch(e){
    console.log("api List faild",e)
    res = {
      

    }
  }
  const arr = await res.json();
  const root = {};
  for (const p of arr) {
    const parts = p.split('/');
    let node = root;
    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const isLast = i === parts.length - 1;
      if (!node[part]) node[part] = { _files: {}, _open: i < 2, _isDir: !isLast };
      node = node[part]._files;
      if (isLast) { node[part] = { _files: {}, _open: false, _isDir: false }; }
    }
  }
  filesEl.innerHTML = '';
  renderTree(filesEl, root, []);

  // Setup file name search with debouncing
  const filterInput = document.getElementById('file-search');
  let debounceTimer;
  filterInput.oninput = () => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      filesEl.innerHTML = '';
      const query = filterInput.value.toLowerCase();
      renderTree(filesEl, root, query.split(/\\s+/).filter(Boolean));
    }, 300); // 300ms debounce
  };
}

function renderTree(rootEl, tree, filters) {
  const ul = document.createElement('ul');
  ul.style.listStyle = 'none';
  ul.style.paddingLeft = '10px';

  for (const [name, meta] of Object.entries(tree)) {
    if (name === '_files') continue;
    const isDir = meta._isDir ?? true;
    const li = document.createElement('li');
    const row = document.createElement('div');
    row.className = 'node';
    const twisty = document.createElement('span');
    twisty.className = 'twisty';
    twisty.textContent = isDir ? (meta._open ? '▼' : '▶') : '•';
    const fname = document.createElement('span');
    fname.className = 'fname';
    fname.textContent = name;

    row.appendChild(twisty);
    row.appendChild(fname);
    li.appendChild(row);
    ul.appendChild(li);

    let passes = true;
    if (filters.length) { const low = name.toLowerCase(); passes = filters.every(f => low.includes(f)); }
    if (!passes) li.style.display = 'none';

    if (isDir) {
      const childRoot = document.createElement('div');
      childRoot.style.paddingLeft = '14px';
      li.appendChild(childRoot);
      const open = !!meta._open;
      childRoot.style.display = open ? 'block' : 'none';
      row.onclick = () => { meta._open = !meta._open; twisty.textContent = meta._open ? '▼' : '▶'; childRoot.style.display = meta._open ? 'block' : 'none'; };
      renderTree(childRoot, meta._files, filters);
    } else { row.onclick = () => openFile(pathJoin(li)); }

    function pathJoin(liNode) {
      const names = [];
      let cur = liNode;
      while (cur && cur.parentElement && cur !== rootEl) {
        const row = cur.querySelector(':scope > .node .fname'); if (row) names.push(row.textContent);
        cur = cur.parentElement.closest('li');
      }
      return names.reverse().join('/');
    }
  }
  rootEl.appendChild(ul);
}

function detectLanguage(path, content) {
  const lower = path.toLowerCase();
  if (lower.endsWith('.rs')) return 'rust';
  if (lower.endsWith('.ts') || lower.endsWith('.tsx')) return 'typescript';
  if (lower.endsWith('.js') || lower.endsWith('.jsx')) return 'javascript';
  if (lower.endsWith('.json')) return 'json';
  if (lower.endsWith('.toml')) return 'toml';
  if (lower.endsWith('.yml') || lower.endsWith('.yaml')) return 'yaml';
  if (lower.endsWith('.md') || lower.endsWith('.markdown')) return 'markdown';
  if (lower.endsWith('.css')) return 'css';
  if (lower.endsWith('.html') || lower.endsWith('.htm')) return 'html';
  if (lower.endsWith('.rest')) return 'http';
  if (lower.endsWith('.env')) return 'ini';
  if (lower.endsWith('.sh') || lower.endsWith('.bash') || lower.endsWith('.zsh') || lower.endsWith('.fish')) return 'shell';
  if (content.startsWith('#!')) {
    if (content.includes('bash') || content.includes('zsh') || content.includes('sh') || content.includes('fish') || content.includes('dash')) return 'shell';
    if (content.includes('python')) return 'python';
  }
  return 'plaintext';
}

async function openFile(p, opts = {}) {
  // Check if file is already open
  if (openFiles.has(p)) {
    switchToFile(p);
    return;
  }

  const url = 'http://127.0.0.1:8789/api/open?' + new URLSearchParams({ path: p });
  const res = await fetch(url);
  const js = await res.json();
  if (js.ok) {
    currentPath = p;
    const content = js.content;
    originalContent = content; // Track original content for dirty check

    // Add to tabs
    openFiles.set(p, {
      content: content,
      cursorPosition: null,
      viewState: null,
      dirty: false
    });
    activeFile = p;

    editor.setValue(content);
    const lang = detectLanguage(p, content);
    try { monaco.editor.setModelLanguage(editor.getModel(), lang); } catch {}
    if (lang === 'markdown' && previewEl.classList.contains('hidden') === false) renderMarkdown();
    if (lang !== 'markdown' && !previewEl.classList.contains('hidden')) togglePreview();

    // restore cursor (unless preserveCursor is explicitly false)
    if (opts.preserveCursor !== false) {
      const s = localStorage.getItem('cursor:'+normalizePath(p));
      if (s) {
        try { const pos = JSON.parse(s); editor.setPosition(pos); editor.revealLineInCenter(pos.lineNumber); } catch {}
      }
    }

    renderFileTabs();
  } else { log(logEl, 'open failed: ' + js.error); }
}

// Track shift key state for copy-on-select behavior
let shiftKeyHeld = false;
document.addEventListener('keydown', (e) => { if (e.key === 'Shift') shiftKeyHeld = true; });
document.addEventListener('keyup', (e) => { if (e.key === 'Shift') shiftKeyHeld = false; });

// Only copy selection when Shift is held during mouse selection
document.addEventListener('mouseup', async (e) => {
  if (!editor) return;
  if (!shiftKeyHeld && !e.shiftKey) return; // Only copy if shift is held
  const sel = editor.getSelection();
  if (!sel || sel.isEmpty()) return;
  const model = editor.getModel();
  const text = model.getValueInRange(sel);
  if (text && navigator.clipboard) {
    try { await navigator.clipboard.writeText(text); setStatus('copied selection'); } catch {}
  }
});





let termLineBuf = '';

//  function ensureTerminalStarted() {
//  if (termStarted) return;
//  termStarted = true;
//  terminal = new window.Terminal({ cursorBlink: true, fontSize: 13, theme: { background: '#0b0e14', foreground: '#cbd5e1' } });
//  const fitAddon = new window.FitAddon.FitAddon();
//  const linkAddon = new window.WebLinksAddon.WebLinksAddon();
//  terminal.loadAddon(fitAddon); terminal.loadAddon(linkAddon);
//  terminal.open(xtermEl);
//  setTimeout(() => { try { fitAddon.fit(); } catch {} }, 0);


function ensureTerminalStarted() {
  if (termStarted) return;
  termStarted = true;
  terminal = new window.Terminal({ cursorBlink: true, fontSize: 13, theme: { background: '#0b0e14', foreground: '#cbd5e1' } });
  fitAddon = new window.FitAddon.FitAddon();
  const linkAddon = new window.WebLinksAddon.WebLinksAddon();
  terminal.loadAddon(fitAddon); terminal.loadAddon(linkAddon);
  terminal.open(xtermEl);
  setTimeout(() => { try { fitAddon.fit(); } catch {} }, 0);
  terminal.onData((d) => {
    // intercept Enter to check for "chat "
    if (d === '\r') {
      const trimmed = termLineBuf.trim();
      if (trimmed.startsWith('chat ')) {
        const prompt = trimmed.slice(5);
        termLineBuf = '';
        terminal.write('\r\n'); // new line
        if (ws && ws.readyState === WebSocket.OPEN) {
          ws.send(JSON.stringify({ type: 'ai_query', prompt }));
        }
        return; // do NOT send to PTY
      }
      termLineBuf = '';
      if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'terminal_input', data: d }));
      return;
    }
    // update buffer for printable chars/backspace
    if (d === '\u007f') { // backspace
      if (termLineBuf.length) termLineBuf = termLineBuf.slice(0, -1);
    } else if (d >= ' ') {
      termLineBuf += d;
    }

    if (ws && ws.readyState === WebSocket.OPEN) ws.send(JSON.stringify({ type: 'terminal_input', data: d }));
  });


  if (ws && ws.readyState === WebSocket.OPEN) {
    const cols = Math.max(80, Math.floor(xtermEl.clientWidth / 9));
    const rows = Math.max(24, Math.floor(xtermEl.clientHeight / 18));
    ws.send(JSON.stringify({ type: 'terminal_start', cols, rows }));
  }



  window.addEventListener('resize', () => {
    if (fitAddon) {
      try { fitAddon.fit(); } catch {}
      if (ws && ws.readyState === WebSocket.OPEN) {
        const cols = Math.max(40, Math.floor(xtermEl.clientWidth / 9));
        const rows = Math.max(10, Math.floor(xtermEl.clientHeight / 18));
        ws.send(JSON.stringify({ type: 'terminal_resize', cols, rows }));
      }
    }
  });

  // Terminal file:line detection with Ctrl+hover
  setupTerminalFileDetection();
}

// Track Ctrl key for terminal file detection
let ctrlKeyHeld = false;
document.addEventListener('keydown', (e) => { if (e.key === 'Control') ctrlKeyHeld = true; });
document.addEventListener('keyup', (e) => { if (e.key === 'Control') ctrlKeyHeld = false; });

// Cache for file existence checks
const fileExistsCache = new Map();

async function checkFileExists(path) {
  if (fileExistsCache.has(path)) {
    return fileExistsCache.get(path);
  }
  try {
    const res = await fetch('http://127.0.0.1:8789/api/exists?' + new URLSearchParams({ path }));
    const data = await res.json();
    fileExistsCache.set(path, data.exists);
    return data.exists;
  } catch {
    return false;
  }
}

function setupTerminalFileDetection() {
  if (!terminal || !xtermEl) return;

  const filePatternRegex = /([a-zA-Z0-9_\-./]+\.[a-z]+):(\d+)(?::(\d+))?/g;
  let currentHighlights = [];

  // Add overlay div for highlights
  const overlay = document.createElement('div');
  overlay.id = 'terminal-overlay';
  overlay.style.cssText = `
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;
    z-index: 10;
  `;
  xtermEl.style.position = 'relative';
  xtermEl.appendChild(overlay);

  async function scanAndHighlightFiles() {
    if (!ctrlKeyHeld || !terminal) {
      // Clear highlights when Ctrl is not held
      overlay.innerHTML = '';
      currentHighlights = [];
      xtermEl.style.cursor = '';
      return;
    }

    xtermEl.style.cursor = 'pointer';
    overlay.innerHTML = '';
    currentHighlights = [];

    const buffer = terminal.buffer.active;
    const viewportY = buffer.viewportY;
    const rows = terminal.rows;

    // Scan visible rows
    for (let row = 0; row < rows; row++) {
      const line = buffer.getLine(viewportY + row);
      if (!line) continue;

      let text = '';
      for (let col = 0; col < line.length; col++) {
        const cell = line.getCell(col);
        if (cell) text += cell.getChars();
      }

      // Find file:line:column patterns
      filePatternRegex.lastIndex = 0;
      let match;
      while ((match = filePatternRegex.exec(text)) !== null) {
        const fullMatch = match[0];
        const filePath = match[1];
        const lineNum = match[2];
        const colNum = match[3] || null;
        const startCol = match.index;

        const exists = await checkFileExists(filePath);

        // Create highlight element
        const highlight = document.createElement('div');
        const charWidth = 9; // approximate char width
        const charHeight = 18; // approximate line height

        highlight.style.cssText = `
          position: absolute;
          left: ${startCol * charWidth}px;
          top: ${row * charHeight}px;
          width: ${fullMatch.length * charWidth}px;
          height: ${charHeight}px;
          background: ${exists ? 'rgba(34, 197, 94, 0.3)' : 'rgba(239, 68, 68, 0.3)'};
          border-bottom: 2px solid ${exists ? '#22c55e' : '#ef4444'};
          pointer-events: auto;
          cursor: pointer;
        `;

        highlight.dataset.path = filePath;
        highlight.dataset.line = lineNum;
        if (colNum) highlight.dataset.col = colNum;
        highlight.dataset.exists = exists;

        // Click handler
        highlight.addEventListener('click', (e) => {
          if (ctrlKeyHeld && e.target.dataset.exists === 'true') {
            const path = e.target.dataset.path;
            const line = parseInt(e.target.dataset.line);
            const col = e.target.dataset.col ? parseInt(e.target.dataset.col) : 1;

            openFile(path, { preserveCursor: false }).then(() => {
              if (editor) {
                editor.setPosition({ lineNumber: line, column: col });
                editor.revealLineInCenter(line);
                editor.focus();
              }
            });
          }
        });

        overlay.appendChild(highlight);
        currentHighlights.push(highlight);
      }
    }
  }

  // Scan when Ctrl is pressed/released
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Control') {
      scanAndHighlightFiles();
    }
  });

  document.addEventListener('keyup', (e) => {
    if (e.key === 'Control') {
      scanAndHighlightFiles();
    }
  });

  // Rescan when terminal content changes
  terminal.onData(() => {
    if (ctrlKeyHeld) {
      setTimeout(scanAndHighlightFiles, 50);
    }
  });
}
  window.addEventListener('keydown', (e) => {
  // Ctrl+S: Save file
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 's') {
    e.preventDefault();
    saveCurrentFile();
  }

  // Ctrl+Shift+P: Command palette
  if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'p') {
    e.preventDefault();
    showCommandPalette();
  }

  // Ctrl+B: Toggle sidebar
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'b') {
    e.preventDefault();
    document.getElementById('toggle-sidebar')?.click();
  }

  // Ctrl+W: Close file
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'w') {
    e.preventDefault();
    currentPath = null;
    if (editor) editor.setValue('// No file open');
  }

  // Ctrl+`: Toggle terminal
  if ((e.ctrlKey || e.metaKey) && e.key === '`') {
    e.preventDefault();
    setTab('terminal');
  }
});

//}

// require(['vs/editor/editor.main', 'vs/basic-languages/toml/toml', 'vs/basic-languages/shell/shell', 'vs/basic-languages/http/http'], function () {

</script>
<!-- <Hello/> -->
 <TopBar {editor} {editorSplit} {splitViewActive}/>
  <div id="container">
   <Sidebar/>
    <div class="divider-vertical" id="sidebar-divider"></div>
    <Main {splitViewActive} {editor} {editorSplit} {splitEditor}/>
  </div>

<div id="ai-modal" class="hidden" style="position:fixed; inset:0; display:flex; align-items:center; justify-content:center; background:rgba(0,0,0,.5);">
 <AIProvider/>
</div>

<CommaondPalette/>

<slot />

