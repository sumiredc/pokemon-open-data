<script lang="ts">
	import type { PokemonRecord } from '$lib/types/data-table.js';
	import DataTable, { Head, Body, Row, Cell, Label, SortValue } from '@smui/data-table';
	import IconButton from '@smui/icon-button';

	export let data: PokemonRecord[];

	let sort: keyof PokemonRecord = 'number';
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
	table$aria-label="Pokemon list"
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
			<Cell columnId="type1" style="width: 100%;">
				<Label>Type1</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="type2" style="width: 100%;">
				<Label>Type2</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="generation" style="width: 100%;">
				<Label>Gen</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="total" style="width: 100%;">
				<Label>Total</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="hp" style="width: 100%;">
				<Label>H</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="attack" style="width: 100%;">
				<Label>A</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="defense" style="width: 100%;">
				<Label>B</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="spAttack" style="width: 100%;">
				<Label>C</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="spDefense" style="width: 100%;">
				<Label>D</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
			<Cell columnId="speed" style="width: 100%;">
				<Label>S</Label>
				<IconButton class="material-icons">arrow_upward</IconButton>
			</Cell>
		</Row>
	</Head>
	<Body>
		{#each data as pokemon (pokemon.id)}
			<Row>
				<Cell numeric>{pokemon.number}</Cell>
				<Cell>{pokemon.name}（{pokemon.englishName}）</Cell>
				<Cell>{pokemon.type1}</Cell>
				<Cell>{pokemon.type2}</Cell>
				<Cell>{pokemon.generation}</Cell>
				<Cell>{pokemon.total}</Cell>
				<Cell>{pokemon.hp}</Cell>
				<Cell>{pokemon.attack}</Cell>
				<Cell>{pokemon.defense}</Cell>
				<Cell>{pokemon.spAttack}</Cell>
				<Cell>{pokemon.spDefense}</Cell>
				<Cell>{pokemon.speed}</Cell>
			</Row>
		{/each}
	</Body>
</DataTable>
