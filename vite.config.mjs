// fsevents-fix:
import path from "node:path";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
// import svelte from "@sveltejs/vite-plugin-svelte";
import { getAliases } from "vite-aliases";

// import { ViteAliases } from 'vite-aliases'
const aliases = getAliases();

// const getAliases = getAliases();
import tailwindcss from "@tailwindcss/vite";
// import tailwindcss from '@tailwindcss/postcss';
// import { svelte } from '@sveltejs/vite-plugin-svelte';
import { sveltePreprocess } from "svelte-preprocess";

// https://vitejs.dev/config/
export default defineConfig(({ mode }) => {
	const isProduction = mode === "production";
	return {
		// plugins: [svelte(), ViteAliases()],
		plugins: [
			tailwindcss(),
			// svelte(),
			getAliases(),
			svelte({
				prebundleSvelteLibraries: true,
				preprocess: sveltePreprocess({}),
			}),
		],
		// fsevents-fix: Tell Vite/esbuild this code is not running on macOS so it can tree-shake fsevents
		define: {
			"process.platform": JSON.stringify(process.platform || "linux"),
		},
		// fsevents-fix: Don’t prebundle fsevents
		optimizeDeps: {
			exclude: ["@roxi/routify", "fsevents", "chokidar"],
		},
		build: {
			minify: isProduction,
			brotliSize: false, // To Speed Up Build
			rollupOptions: {
				// fsevents-fix: Keep fsevents out of the Rollup build
				external: ["fsevents"],
				// Disabled Hashing as Netlify Does Hashing for us using Etag.
				output: {
					entryFileNames: `assets/[name].js`,
					chunkFileNames: `assets/[name].js`,
					assetFileNames: `assets/[name].[ext]`,
				},
			},
		},
		resolve: {
			alias: aliases,
			// fsevents-fix: Last-resort shim (if something still imports it)
			fsevents: path.resolve(__dirname, "src/shims/empty.js"),
		},
	};
});
//export default {
//	plugins: [
//		ViteAliases()
//	]
//};
