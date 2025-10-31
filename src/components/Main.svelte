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