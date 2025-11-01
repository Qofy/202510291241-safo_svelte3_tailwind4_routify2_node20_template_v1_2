<script>
	import SplitViewToggle from "./btn/SplitViewToggle.svelte";
	import Tabs from "./Tabs.svelte";

  export let splitViewActive = false
  export let editor = {}
export let editorSplit ={}
export let splitEditor = {}
</script>



<div id="main">
      <div id="editor-area">
        <div id="editor-tabs">
        <SplitViewToggle {splitViewActive} {editor} {editorSplit} {splitEditor}/>

        </div>
        <div id="editor-container">
          <div id="editor"></div>
          <div class="divider-vertical" id="editor-split-divider" style="display:none;"></div>
          <div id="editor-split"></div>
        </div>
        <div id="preview" class="hidden"></div>
      </div>
      <div class="divider-horizontal" id="bottom-divider"></div>
      <div id="bottom">
        <Tabs/>
        <div id="panel-lsp" class="panel"><div id="log"></div></div>
        <div id="panel-runner" class="panel"><div id="termlog"></div></div>
        <div id="panel-terminal" class="panel"><div id="xterm"></div></div>
        <div id="panel-search" class="panel"><div id="search-results"></div></div>
        <div id="panel-problems" class="panel"><div id="problems"></div></div>
        <div id="panel-log" class="panel"></div>
        <div id="panel-xterm" class="panel"></div>
        <div id="panel-termlog" class="panel"></div>
      </div>
    </div>

    <style>
      
        #main { display: grid; grid-template-rows: 1fr 260px; background: var(--bg2); }
    #editor-area { display: flex; flex-direction: column; }

      #editor-tabs::-webkit-scrollbar { height: 6px; }
    #editor-tabs::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
    /* .file-tab { display: flex; align-items: center; gap: 6px; padding: 6px 12px; border: 1px solid var(--border); border-radius: 6px 6px 0 0; background: #111724; cursor: pointer; font-size: 13px; transition: all 0.2s; user-select: none; position: relative; }
    .file-tab:hover { background: #1a2138; }
    .file-tab.active { background: var(--bg2); border-bottom-color: var(--bg2); color: var(--accent); }
    .file-tab .save-indicator { width: 8px; height: 8px; border-radius: 50%; background: #fbbf24; display: none; }
    .file-tab.dirty .save-indicator { display: block; }
    .file-tab .close-btn { opacity: 0; margin-left: 4px; padding: 0 4px; cursor: pointer; }
    .file-tab:hover .close-btn { opacity: 1; }
    .file-tab .close-btn:hover { color: #ef4444; } */
     #editor-container { flex: 1; display: grid; grid-template-columns: 1fr 0; position: relative; }
     #editor { height: 100%; }
     #editor { flex: 1 1 auto; min-width: 0; }
     /* Resizable dividers */
    .divider-vertical {
      width: 4px;
      background: transparent;
      cursor: col-resize;
      position: relative;
      transition: background 0.2s;
    }

    .divider-vertical:hover {
      background: var(--accent);
    }

    #editor-split { height: 100%; display: none; }
#preview { border-left: 1px solid var(--border); padding: 12px; overflow: auto; background:#0a0f17; }
    .hidden { display:none !important; }

      .divider-horizontal {
      height: 4px;
      background: transparent;
      cursor: row-resize;
      position: relative;
      transition: background 0.2s;
    }

     .divider-horizontal:hover {
      background: var(--accent);
    }

 #bottom { border-top: 1px solid var(--border); background: #0c111b; display: grid; grid-template-rows: 36px 1fr; }

 .panel {
  display: none;
  overflow: auto;
  opacity: 0;
  transition: opacity 0.2s ease;
}

    </style>