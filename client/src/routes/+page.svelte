<script lang="ts">
	import DataTable from '$lib/components/DataTable.svelte';
	import { Api } from '$lib/api/v1.js';

	const api = new Api();

	async function getPokemonList() {
		try {
			const { data } = await api.pokemon.pokemonList();

			return data.data;
		} catch (error) {
			console.log(error);
			throw new Error('データの取得に失敗しました');
		}
	}
</script>

<h1>Pokemon Open Data</h1>

{#await getPokemonList()}
	<p>...waiting</p>
{:then pokemonList}
	<DataTable data={pokemonList} />
{:catch error}
	<p style="color: red">{error.message}</p>
{/await}
