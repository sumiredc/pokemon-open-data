import type { Pokemon } from '$lib/api/v1.js';
import type { PokemonRecord } from '$lib/types/data-table.js';
import { toUpperCaseFirst } from '$lib/utils/string.js';

/**
 * @singleton
 */
export class DataTableDTOs {
	private static _instance: DataTableDTOs;

	private constructor() {}

	static new() {
		if (DataTableDTOs._instance === undefined) {
			return new DataTableDTOs();
		}
		return DataTableDTOs._instance;
	}

	toPokemonRecord(pokemon: Pokemon): PokemonRecord {
		return {
			id: pokemon.id,
			number: pokemon.number,
			name: pokemon.name,
			englishName: pokemon.englishName,
			type1: toUpperCaseFirst(pokemon.type1.name),
			type2: toUpperCaseFirst(pokemon.type2?.name ?? ''),
			generation: this.toGenerationLabel(pokemon.generation.value),
			total: Object.values(pokemon.stats).reduce((p, c) => p + c, 0),
			...pokemon.stats
		};
	}

	private toGenerationLabel(generation: number) {
		switch (generation) {
			case 1:
				return '1st';
			case 2:
				return '2nd';
			case 3:
				return '3rd';
			default:
				return `${generation}th`;
		}
	}
}
