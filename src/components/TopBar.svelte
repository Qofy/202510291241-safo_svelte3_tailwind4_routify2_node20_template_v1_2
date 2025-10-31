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

  export let ws = null

</script>

 <div id="topbar">
    <ButtonToggleSlider {editor} {editorSplit} {splitViewActive}/>
    <ButtonRust {ws}/>
    <ButtonTs {ws}/>
    <ButtonCheck {ws}/>
    <SaveButton/>
    <ButtonClear/>
    <PreviewButton/>
    <span class="tag">Workspace</span><span id="ws-path" class="dim"></span>
    <span id="status" class="status">—</span>
  </div>