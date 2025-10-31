<style>


	main {
		text-align: center;
		padding: 1em;
		margin: 0 auto;
	}

	img {
		height: 16rem;
		width: 16rem;
	}
	p {
		max-width: 14rem;
		margin: 1rem auto;
		line-height: 1.35;
	}

	@media (min-width: 480px) {
		h1 {
			max-width: none;
		}

		p {
			max-width: none;
		}
	}
</style>

<main class="flex flex-col items-center">
	<img src="./assets/svelte.png" alt="Svelte Logo" />
	<h1 class="mb-5 text-5xl font-light text-red-500 uppercase">
		Hallo
	</h1>

	<slot />

	<p>
		Besuch <a class="hover:underline" href="https://bucksmanage.dev" aria-label="Geh nach bucksmanage.berlin um Fahrrad zu lernen."
			>bucksmanage.berlin</a> .
	</p>

	<p>
		Geh nach
		<a aria-label="Geh nach bucksmanage.berlin/nach bucksmanage.berlin/kaufen.."
			class="hover:underline"
			href="https://bucksmanage.berlin/fahrraeder">
			bucksmanage.berlin/fahrraeder
		</a>
		um Fahrrader zu kaufen.
	</p>
</main>
