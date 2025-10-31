<script>
	import ButtonFmt from "./btn/ButtonFmt.svelte";
	import ButtonGear from "./btn/ButtonGear.svelte";
	import ButtonRestart from "./btn/ButtonRestart.svelte";
	import ButtonRunnerStart from "./btn/ButtonRunnerStart.svelte";
	import ButtonRunnerStop from "./btn/ButtonRunnerStop.svelte";
	import ToggleButton from "./btn/ToggleButton.svelte";


</script>

 <div id="tabs">
          <ToggleButton/>
          <div id="tab-lsp" class="tab active" data-tab="lsp">LSPsd</div>
          <div id="tab-runner" class="tab" data-tab="runner">Runner</div>
          <div id="tab-terminal" class="tab" data-tab="terminal">Terminal</div>
          <div id="tab-search" class="tab" data-tab="search">Search Results</div>
          <div id="tab-problems" class="tab" data-tab="problems">Problems</div>
          <div id="tab-log" class="tab" data-tab="log">Log</div>
          <div id="tab-xterm" class="tab" data-tab="xterm">Xterm</div>
          <div id="tab-termlog" class="tab" data-tab="termlog">Xtermlog</div>
          <ButtonGear/>
          <ButtonRunnerStart/>
          <ButtonRestart/>
          <ButtonRunnerStop/>
         <ButtonFmt/>
        </div>