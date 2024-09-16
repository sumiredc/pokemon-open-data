<script lang="ts">
	import type { Pokemon } from '$lib/api/v1.js';
	import DataTable, { Head, Body, Row, Cell, Label, SortValue } from '@smui/data-table';
	import IconButton from '@smui/icon-button';

	export let data: Pokemon[];

	let sort: keyof Pokemon = 'number';
	let sortDirection: Lowercase<keyof typeof SortValue> = 'ascending';

	function handleSort() {
		data.sort((a, b) => {
			const [aVal, bVal] = [a[sort], b[sort]][
				sortDirection === 'ascending' ? 'slice' : 'reverse'
			]();
			if (typeof aVal === 'string' && typeof bVal === 'string') {
				return aVal.localeCompare(bVal);
			}
			return Number(aVal) - Number(bVal);
		});
		data = data;
	}
</script>

<DataTable
	sortable
	bind:sort
	bind:sortDirection
	on:SMUIDataTable:sorted={handleSort}
	table$aria-label="User list"
	style="width: 100%;"
>
	<Head>
		<Row>
			<Cell numeric columnId="number">
				<Label>Number</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="name" style="width: 100%;">
				<Label>Name</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="type1.name" style="width: 100%;">
				<Label>Type1</Label>
			</Cell>
			<Cell columnId="type2.name" style="width: 100%;">
				<Label>Type2</Label>
			</Cell>
		</Row>
	</Head>
	<Body>
		{#each data as pokemon (pokemon.number)}
			<Row>
				<Cell numeric>{pokemon.number}</Cell>
				<Cell>{pokemon.name}({pokemon.englishName})</Cell>
				<Cell>{pokemon.type1.name}</Cell>
				<Cell>{pokemon.type2?.name ?? ''}</Cell>
			</Row>
		{/each}
	</Body>
</DataTable>
