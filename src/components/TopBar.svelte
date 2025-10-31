<script>
	// import { editor } from "monaco-editor";
  export let editor = {}
  export let editorSplit={}
  export let splitViewActive=false
	import ButtonCheck from "./btn/ButtonCheck.svelte";
	import ButtonClear from "./btn/ButtonClear.svelte";
	import ButtonRust from "./btn/ButtonRust.svelte";
	import ButtonToggleSlider from "./btn/ButtonToggleSlider.svelte";
	import ButtonTs from "./btn/ButtonTs.svelte";
	import PreviewButton from "./btn/PreviewButton.svelte";
	import SaveButton from "./btn/SaveButton.svelte";

 export let currentPath = null;
  export let ws = null;
  let terminal = false

</script>

 <div id="topbar">
    <ButtonToggleSlider {editor} {editorSplit} {splitViewActive}/>
    <ButtonRust {ws}/>
    <ButtonTs {ws}/>
    <ButtonCheck {ws}/>
    <SaveButton {currentPath}/>
    <ButtonClear {terminal}/>
    <PreviewButton {currentPath}/>
    <span class="tag">Workspace</span><span id="ws-path" class="dim"></span>
    <span id="status" class="status">—</span>
  </div>

  <style>
     #topbar { height: 44px; display: flex; gap: 8px; align-items: center; padding: 0 12px; border-bottom: 1px solid var(--border); background: linear-gradient(180deg, #12151d, #0f121a); position: sticky; top: 0; z-index: 10; }
    /* button { padding: 6px 10px; border-radius: 8px; border: 1px solid var(--border); background: var(--btn); color: var(--text); cursor: pointer; }
    button:hover { background: var(--btnh); }
    .tag { padding: 3px 8px; border:1px solid var(--border); border-radius:999px; color: var(--muted); } */
     .tag { padding: 3px 8px; border:1px solid var(--border); border-radius:999px; color: var(--muted); }
     .dim { color: var(--muted); }
       .status { margin-left: auto; color: var(--muted); }
  </style>