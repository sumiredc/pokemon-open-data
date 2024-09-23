<script lang="ts">
	import Paper, { Title, Content } from '@smui/paper';
	import DataTable from '$lib/components/DataTable.svelte';
	import LayoutGrid, { Cell } from '@smui/layout-grid';
	import Chart from '$lib/components/Chart.svelte';
	import { PokemonService } from '$lib/api/services/pokemon-service.js';
	import { SummaryService } from '$lib/api/services/summary-service.js';

	const pokemonService = PokemonService.new();
	const summaryService = SummaryService.new();
</script>

<h1>Pokemon Open Data</h1>

<LayoutGrid>
	<Cell>
		{#await summaryService.getType()}
			<p>...waiting</p>
		{:then dataSources}
			<Chart {dataSources} title="Type" />
		{:catch error}
			<Paper color="primary" variant="outlined" class="mdc-theme--primary">
				<Title>Data fetch error</Title>
				<Content>{error.message}</Content>
			</Paper>
		{/await}
	</Cell>
	<Cell>
		{#await summaryService.getGeneration()}
			<p>...waiting</p>
		{:then dataSources}
			<Chart {dataSources} title="Generation" />
		{:catch error}
			<Paper color="primary" variant="outlined" class="mdc-theme--primary">
				<Title>Data fetch error</Title>
				<Content>{error.message}</Content>
			</Paper>
		{/await}
	</Cell>
	<Cell>
		{#await summaryService.getStats()}
			<p>...waiting</p>
		{:then dataSources}
			<Chart {dataSources} title="Stats" />
		{:catch error}
			<Paper color="primary" variant="outlined" class="mdc-theme--primary">
				<Title>Data fetch error</Title>
				<Content>{error.message}</Content>
			</Paper>
		{/await}
	</Cell>
</LayoutGrid>

{#await pokemonService.getList()}
	<p>...loading</p>
{:then pokemonList}
	<LayoutGrid>
		<Cell span={12}>
			<DataTable data={pokemonList} />
		</Cell>
	</LayoutGrid>
{:catch error}
	<Paper color="primary" variant="outlined" class="mdc-theme--primary">
		<Title>Data fetch error</Title>
		<Content>{error.message}</Content>
	</Paper>
{/await}
