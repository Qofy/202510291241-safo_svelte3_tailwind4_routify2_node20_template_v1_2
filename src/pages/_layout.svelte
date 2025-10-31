<script context="module">
	export const prerender = true;
	// export let scoped = "";
	// export let scopedSync = "";
</script>

<script>

	// export let scoped = "";
	// export let scopedSync = "";
	import {
		getWebServerRelativeUrl,
		getWebServerSearchUrl,
	} from "../components/common/types/_functions.js";
	// export let scoped = "";
	// export let scopedSync = "";
	let path = getWebServerRelativeUrl();
	let search = getWebServerSearchUrl();
	let service;
	if (("_" + search).indexOf("?s=") > 0) {
		service = "results";
	}
	if (("_" + search).indexOf("?q=") > 0) {
		service = "results";
	}
</script>

<style global>
	@import "tailwindcss";

	@tailwind base;
	@tailwind components;
	@tailwind utilities;
	/* Paste all your custom animations & overrides here: */
<style>
	.layout {
		/* Keep the size reasonable, so the large monitor users not getting bad website designs*/
		margin: 0 auto;
	}
</style>

<div class=" px-0 mt-16 layout h-screen">
	<div class="container mx-auto">
		<div class="e404">
			<div class="big"></div>
		</div>
	</div>
	<slot />
	<!-- _hash_me_ gets loaded here after  -->
</div>
