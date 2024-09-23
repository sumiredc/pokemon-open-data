import { DataTableDTOs } from '$lib/dtos/data-table.js';
import { Api, type Pokemon } from '../v1.js';

/**
 * @singleton
 */
export class PokemonService {
	private static _instance: PokemonService;

	private constructor(
		private api: Api<unknown>,
		private dataTableDtos: DataTableDTOs
	) {}

	static new() {
		if (PokemonService._instance === undefined) {
			return new PokemonService(new Api(), DataTableDTOs.new());
		}
		return PokemonService._instance;
	}

	async getList() {
		try {
			const { data } = await this.api.pokemon.pokemonList();
			return data.map((pokemon: Pokemon) => this.dataTableDtos.toPokemonRecord(pokemon));
		} catch (error) {
			console.error(error);
			throw new Error('データの取得に失敗しました');
		}
	}
}
