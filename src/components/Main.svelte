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
      <!-----------bottom------------>
      <div id="bottom">
        <Tabs/>
        <div id="panel-lsp" class="panel">
          <div id="log"></div>
        </div>

        <div id="panel-runner" class="panel">
          <div id="termlog"></div>
        </div>

        <div id="panel-terminal" class="panel">
          <div id="xterm"></div>
        </div>

        <div id="panel-search" class="panel">
          <div id="search-results"></div>
        </div>

        <div id="panel-problems" class="panel">
          <div id="problems"></div>
        </div>

        <div id="panel-log" class="panel"></div>
        <div id="panel-xterm" class="panel"></div>
        <div id="panel-termlog" class="panel"></div>
      </div>
    </div>

<style>
#main { display: grid; grid-template-rows: 1fr 260px; background: var(--bg2); }
#editor-area { display: flex; flex-direction: column; }
#editor-tabs { display: flex; gap: 4px; align-items: center; padding: 6px 8px; border-bottom: 1px solid var(--border); background: #0e1420; overflow-x: auto; }
#editor-tabs::-webkit-scrollbar { height: 6px; }
#editor-tabs::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
#editor { height: 100%; }
#editor { flex: 1 1 auto; min-width: 0; }
#editor-container { flex: 1; display: grid; grid-template-columns: 1fr 0; position: relative; }
#editor { height: 100%; }
#editor-split { height: 100%; display: none; }
#bottom { border-top: 1px solid var(--border); background: #0c111b; display: grid; grid-template-rows: 36px 1fr; }
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
.panel {
  display: none;
  overflow: auto;
  opacity: 0;
  transition: opacity 0.2s ease;
  height: 100%; /* Add this */
}
/* .panel.active {
  display: block;
  opacity: 1;
} */
/* Bottom panel collapse states */
#bottom { transition: grid-template-rows 0.3s ease; }
#log, #termlog, #xterm, #search-results { 
  height: 100%; /* Changed from min-height: 20rem */
  padding: 8px; 
  overflow: auto; 
  font-family: ui-monospace, SFMono-Regular, Menlo, monospace; 
  font-size: 12px; 
  white-space: pre-wrap; 
}
#xterm { 
  padding: 0; /* Keep xterm without padding */
}
/* When panels are hidden, remove height */
.panel:not(.active) #log,
.panel:not(.active) #termlog,
.panel:not(.active) #xterm,
.panel:not(.active) #search-results {
  height: 0;
  min-height: 0;
}
</style>