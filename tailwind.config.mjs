// tailwind.config.mjs
import config from '@tailwindcss/vite';
import { defineConfig } from 'tailwindcss'

export default defineConfig({
	content: ["./src/**/*.{html,vue,svelte,js,jsx,tsx}"],
	darkMode: false, // Can also be 'media' or 'class'
	theme: {
		extend: {
			fontFamily: {
				"executorcom-playfair-display-bold-50-font-family":
					"PlayfairDisplay-Bold, sans-serif",
				// [continued, keep all fontFamily entries unchanged...]
				"executorcom-montserrat-extralight-18-title-font-family":
					"Montserrat-ExtraLight, sans-serif",
			},
			fontSize: {
				h1: "var(--h1)",
				h2: "var(--h2)",
				h3: "var(--h3)",
				h4: "var(--h4)",
				h5: "var(--h5)",
				h6: "var(--h6)",
				// [continued, keep all fontSize entries unchanged...]
				"executorcom-montserrat-extralight-18-title-font-size": "18px",
			},
			fontWeight: {
				// [same as original...]
				"executorcom-montserrat-extralight-18-title-font-weight": "200",
			},
			lineHeight: {
				// [same as original...]
				"executorcom-montserrat-extralight-18-title-line-height": "25.2px",
			},
			letterSpacing: {},
			borderRadius: {
				"4xl": "2rem",
			},
			colors: {
				black: "var(--black)",
				"dark-grey": "var(--dark-grey)",
				"light-grey": "var(--light-grey)",
				yellow: "#fbb900",
				divider: "var(--divider)",
				"sand-dark": "var(--sand-dark)",
				"sand-light": "var(--sand-light)",
				"off-white": "var(--off-white)",
				white: "var(--white)",
				red: {
					DEFAULT: "#c51a1b",
					300: "rgba(252, 165, 165, var(--tw-bg-opacity))",
					400: "rgba(248, 113, 113, var(--tw-bg-opacity))",
				},
				gray: {
					900: "var(--black)",
					800: "var(--dark-grey)",
					700: "var(--light-grey)",
					400: "var(--divider)",
					300: "var(--sand-dark)",
					200: "var(--sand-light)",
					100: "var(--off-white)",
				},
				orange: {
					900: "var(--brand-almost-ripe)",
					800: "var(--brand-ripe)",
					700: "var(--brand-light)",
				},
				pink: {
					900: "var(--salmon)",
				},
				"executorcom-mine-shaft": "#202020",
				"executorcom-wild-strawberry": "#fc2981",
				"executorcom-nero": "#ffffff",
				"executorcom-boulder": "#7a7a7a",
				"executorcom-rose-of-sharon": "#b55fff",
				"executorcom-black": "#000000",
				"executorcom-gallery": "#eeeeee",
				"executorcom-wild-strawberry-36percent": "rgba(252, 41, 129, 0.36)",
				"executorcom-wild-strawberry-60percent": "rgba(252, 41, 129, 0.60)",
				"executorcom-concrete": "#f2f2f2",
				"executorcom-black-haze": "#f9fafa",
				"executorcom-tundora": "#414141",
				"executorcom-abbey": "#54595f",
				"executorcom-mariner": "#1e73be",
				"executorcom-alto": "#dddddd",
				"executorcom-fiord": "#3d4e60",
			},
			width: {},
			minWidth: {},
			height: {},
			minHeight: {},
			maxHeight: {},
			backgroundImage: {
				"gitpod-kumquat-gradient":
					"linear-gradient(137.41deg, #FFAD33 14.37%, #FF8A00 91.32%)",
			},
			maxWidth: {
				row: "var(--row-max-width)",
			},
			spacing: {
				"xx-large": "var(--xx-large)",
				"x-large": "var(--x-large)",
				large: "var(--large)",
				medium: "var(--medium)",
				small: "var(--small)",
				"x-small": "var(--x-small)",
				"xx-small": "var(--xx-small)",
				micro: "var(--micro)",
				macro: "var(--macro)",
			},
			screens: {
				xsm: "635px",
				sm: "640px",
				md: "768px",
				lg: "1024px",
				xl: "1280px",
				"2xl": "1536px",
			},
		},
	},
	plugins: [],
});

