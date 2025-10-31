<script>

</script>

<div id="command-palette" class="hidden">
  <div id="command-palette-backdrop"></div>
  <div id="command-palette-container">
    <div id="command-palette-header">
      <input id="command-palette-input" type="text" placeholder="Type a command or search..." autofocus />
      <div id="command-palette-hint">Press <kbd>↑</kbd> <kbd>↓</kbd> to navigate, <kbd>Enter</kbd> to execute, <kbd>Esc</kbd> to close</div>
    </div>
    <div id="command-palette-results"></div>
  </div>
</div>

<style>
  /* Command Palette */
#command-palette {
  position: fixed;
  inset: 0;
  z-index: 2000;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 15vh;
  animation: fadeIn 0.15s ease;
}

#command-palette.hidden {
  display: none;
}

#command-palette-backdrop {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(2px);
}

#command-palette-container {
  position: relative;
  width: 90%;
  max-width: 640px;
  background: var(--panel);
  border: 1px solid var(--accent);
  border-radius: 12px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6), 0 0 0 1px rgba(97, 218, 251, 0.1);
  overflow: hidden;
  animation: slideDown 0.2s ease;
}


#command-palette-header {
  border-bottom: 1px solid var(--border);
  background: var(--bg);
}

#command-palette-input {
  width: 100%;
  padding: 18px 20px;
  background: transparent;
  border: none;
  color: var(--text);
  font-size: 16px;
  font-family: inherit;
  outline: none;
}

#command-palette-input::placeholder {
  color: var(--muted);
}

#command-palette-hint {
  padding: 8px 20px 12px;
  font-size: 12px;
  color: var(--muted);
  display: flex;
  align-items: center;
  gap: 8px;
}

#command-palette-hint kbd {
  background: var(--btn);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 2px 6px;
  font-family: monospace;
  font-size: 11px;
  color: var(--text);
  box-shadow: 0 1px 2px rgba(0,0,0,0.2);
}

#command-palette-results {
  max-height: 60vh;
  overflow-y: auto;
  padding: 8px 0;
}
</style>