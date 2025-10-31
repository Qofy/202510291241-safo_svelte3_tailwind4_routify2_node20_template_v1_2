<script>
	import { url } from "@roxi/routify";
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
	$: console.log(
		"_fallback.svelte path:",
		{ path },
		" service:",
		{ service },
		" service:",
		{ service },
	);
</script>

<style>
	.huge {
		font-size: 12rem;
	}
	.e404 {
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		text-align: center;
	}
</style>

<div class="e404">
	<div class="huge">404</div>
	<div class="big">
		Page not found.
		<!-- link to the parent folder of _fallback.svelte -->
		<a href="{$url('../')}" aria-label="Go back">Go back</a>
	</div>
</div>
