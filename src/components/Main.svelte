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
 #main { display: flex; flex-direction: column; background: var(--bg2); height: 100%; overflow: hidden; }
    :global(#main.bottom-panel-collapsed) #editor-area { margin-bottom: 0; }
    #editor-area { display: flex; flex-direction: column; flex: 1; min-height: 0; margin-bottom: 0px; transition: margin-bottom 0.3s ease; }
    #editor-tabs { display: flex; gap: 4px; align-items: center; padding: 6px 8px; border-bottom: 1px solid var(--border); background: #0e1420; overflow-x: auto; flex-shrink: 0; }
    #editor-tabs::-webkit-scrollbar { height: 6px; }
    #editor-tabs::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
     #editor-container { flex: 1; display: grid; grid-template-columns: 1fr 0; position: relative; min-height: 0; }
    #editor { height: 100%; }
    #editor-split { height: 100%; display: none; }
    #preview { border-left: 1px solid var(--border); padding: 12px; overflow: auto; background:#0a0f17; }
    #bottom { border-top: 1px solid var(--border); background: #0c111b; display: grid; grid-template-rows: 36px 1fr; height: 260px; flex-shrink: 0; z-index: 10; transition: height 0.3s ease; }
    :global(#bottom.collapsed) { height: 36px; grid-template-rows: 36px 0; }
    :global(#main.bottom-panel-collapsed) #editor-area { margin-bottom: 0px; }
.panel {
  display: none;
  overflow: auto;
  opacity: 0;
  transition: opacity 0.2s ease;
}

:global(.panel.active) {
  display: block !important;
  opacity: 1;
}
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
</style>