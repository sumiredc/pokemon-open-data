import { ChartDTOs } from '$lib/dtos/chart.js';
import { Api, type Summary } from '../v1.js';

/**
 * @singleton
 */
export class SummaryService {
	private static _instance: SummaryService;

	private constructor(
		private api: Api<unknown>,
		private chartDtos: ChartDTOs
	) {}

	static new() {
		if (SummaryService._instance === undefined) {
			return new SummaryService(new Api(), ChartDTOs.new());
		}
		return SummaryService._instance;
	}

	async getType() {
		try {
			const { data } = await this.api.summary.typeList();

			return data.map((summary: Summary) => this.chartDtos.toTypeDataSource(summary));
		} catch (error) {
			console.error(error);
			throw new Error('データの取得に失敗しました');
		}
	}

	async getGeneration() {
		try {
			const { data } = await this.api.summary.generationList();

			return data.map((summary: Summary) => this.chartDtos.toGenerationDataSource(summary));
		} catch (error) {
			console.error(error);
			throw new Error('データの取得に失敗しました');
		}
	}

	async getStats() {
		try {
			const { data } = await this.api.summary.statsList();

			return data.map((summary: Summary) => this.chartDtos.toStatsDataSource(summary));
		} catch (error) {
			console.error(error);
			throw new Error('データの取得に失敗しました');
		}
	}
}
