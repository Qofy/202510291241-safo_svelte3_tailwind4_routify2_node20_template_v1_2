<script>
	import ButtonFmt from "./btn/ButtonFmt.svelte";
	import ButtonGear from "./btn/ButtonGear.svelte";
	import ButtonRestart from "./btn/ButtonRestart.svelte";
	import ButtonRunnerStart from "./btn/ButtonRunnerStart.svelte";
	import ButtonRunnerStop from "./btn/ButtonRunnerStop.svelte";
	import ToggleButton from "./btn/ToggleButton.svelte";

let ws = null;
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
          <ButtonGear {ws}/>
          <ButtonRunnerStart {ws}/>
          <ButtonRestart {ws}/>
          <ButtonRunnerStop {ws}/>
         <ButtonFmt {ws}/>
        </div>

        <style>
              .tab {
      padding: 6px 12px;
      border: 1px solid var(--border);
      border-radius: 8px;
      background: #111724;
      cursor: pointer;
      font-size: 13px;
      font-weight: 500;
      transition: all 0.2s ease;
      user-select: none;
      position: relative;
    }
    .tab:hover {
      background: #1a2138;
      border-color: var(--accent);
      transform: translateY(-1px);
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
    }
    .tab.active {
      background: #172034;
      color: var(--accent);
      border-color: var(--accent);
      box-shadow: 0 2px 4px rgba(82, 178, 255, 0.2);
    }
    .tab.active::after {
      content: '';
      position: absolute;
      bottom: -7px;
      left: 50%;
      transform: translateX(-50%);
      width: 60%;
      height: 2px;
      background: var(--accent);
      border-radius: 2px;
    }
        </style>