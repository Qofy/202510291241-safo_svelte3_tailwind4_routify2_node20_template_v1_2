<script>
	import SearchButton from "./btn/SearchButton.svelte";


</script>


<div id="sidebar">
      <div id="sidebar-top">
        <input id="file-search" type="text" placeholder="Search files by name…" />
        <SearchButton/>
      </div>
      <ul id="files"></ul>
    </div>