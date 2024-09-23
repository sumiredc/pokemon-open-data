import type { Summary } from '$lib/api/v1.js';
import { GenerationColor, StatsColor, TypeColor } from '$lib/consts/color.js';
import type { DataSource } from '$lib/types/chart.js';
import { toUpperCaseFirst } from '$lib/utils/string.js';

/**
 * @singleton
 */
export class ChartDTOs {
	private static _instance: ChartDTOs;

	private generationKeyMap: { [k: string]: string } = {
		'1': 'First',
		'2': 'Second',
		'3': 'Third',
		'4': 'Fourth',
		'5': 'Fifth',
		'6': 'Sixth',
		'7': 'Seventh',
		'8': 'Eighth',
		'9': 'Ninth'
	};

	private statsKeyMap: { [k: string]: string } = {
		'~200': 'Stat_0_200',
		'201~300': 'Stat_201_300',
		'301~400': 'Stat_301_400',
		'401~500': 'Stat_401_500',
		'501~600': 'Stat_501_600',
		'601~700': 'Stat_601_700',
		'701~': 'Stat_701_800'
	};

	private constructor() {}

	static new() {
		if (ChartDTOs._instance === undefined) {
			return new ChartDTOs();
		}
		return ChartDTOs._instance;
	}

	toTypeDataSource(summary: Summary): DataSource {
		const label = toUpperCaseFirst(summary.label);
		const colorKey = this.typeColorKey(label);
		const color = TypeColor[colorKey];

		return {
			value: summary.value,
			label,
			color
		};
	}

	private typeColorKey(label: string): keyof typeof TypeColor {
		if (Object.keys(TypeColor).includes(label)) {
			return label as keyof typeof TypeColor;
		}
		return 'Default';
	}

	toGenerationDataSource(summary: Summary): DataSource {
		const label = this.generationKeyMap[summary.label];
		const colorKey = this.generationColorKey(label);
		const color = GenerationColor[colorKey];

		return {
			value: summary.value,
			label,
			color
		};
	}

	private generationColorKey(label: string): keyof typeof GenerationColor {
		if (Object.keys(GenerationColor).includes(label)) {
			return label as keyof typeof GenerationColor;
		}
		return 'Default';
	}

	toStatsDataSource(summary: Summary): DataSource {
		const key = this.statsKeyMap[summary.label];
		const colorKey = this.statsColorKey(key);
		const color = StatsColor[colorKey];

		return {
			...summary,
			color
		};
	}

	private statsColorKey(label: string): keyof typeof StatsColor {
		if (Object.keys(StatsColor).includes(label)) {
			return label as keyof typeof StatsColor;
		}
		return 'Default';
	}
}
