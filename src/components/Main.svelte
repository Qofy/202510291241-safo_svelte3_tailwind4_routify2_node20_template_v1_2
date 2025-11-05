
  <div id="main">
      <div id="editor-area">
        <div id="editor"></div>
        <div id="preview" class="hidden"></div>
      </div>
      <div id="bottom">
        <div id="tabs">
          <div class="tab active" data-tab="lsp">LSP</div>
          <div class="tab" data-tab="terminal">Terminal</div>
          <div class="tab" data-tab="runner">Runner</div>
          <button id="runner-start">Start</button>
          <button id="runner-restart">Restart</button>
          <button id="runner-stop">Stop</button>
        </div>
        <div id="log"></div>
        <div id="xterm"></div>
        <div id="termlog"></div>
      </div>
    </div>

    <style>
   
  
   
    #main { display: grid; grid-template-rows: 1fr 260px; background: var(--bg2); }
    #editor-area { display: grid; grid-template-columns: 1fr 0; }
    #editor { height: 100%; }
    #preview { border-left: 1px solid var(--border); padding: 12px; overflow: auto; background:#0a0f17; }
    #bottom { border-top: 1px solid var(--border); background: #0c111b; display: grid; grid-template-rows: 36px 1fr; }
    #tabs { display: flex; gap: 6px; align-items: center; padding: 6px 8px; border-bottom: 1px solid var(--border); background: #0e1420; }
    .tab { padding:6px 10px; border:1px solid var(--border); border-radius:8px; background:#111724; cursor:pointer; }
    .tab.active { background:#172034; color: var(--accent); }
    #log, #termlog, #xterm { padding: 8px; overflow: auto; font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 12px; white-space: pre-wrap; }
    #log { display: block; }
    #termlog { display: none; }
    #xterm { display:none; height: 100%; padding: 0; }
   
    </style>