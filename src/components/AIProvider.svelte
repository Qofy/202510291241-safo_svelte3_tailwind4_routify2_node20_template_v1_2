<script>
	import AiCancelButton from "./btn/AiCancelButton.svelte";
	import AiSaveButton from "./btn/AiSaveButton.svelte";

</script>


 <div style="background:#0e1420; border:1px solid var(--border); border-radius:12px; padding:16px; width:420px;">
    <h3>AI Provider</h3>
    <label>Provider
      <select id="ai-provider">
        <option value="ollama">ollama</option>
        <option value="openai">openai</option>
        <option value="claude">claude</option>
        <option value="deepseek">deepseek</option>
      </select>
    </label>
    <label>Model <input id="ai-model" placeholder="e.g. llama3.1:8b or gpt-4o-mini" style="width:100%"></label>
    <label>API Key (if needed) <input id="ai-key" type="password" style="width:100%"></label>
    <div style="display:flex; gap:8px; justify-content:flex-end; margin-top:12px;">
      <AiCancelButton/>
      <AiSaveButton/>
    </div>
  </div>

  <style>
    
  </style>