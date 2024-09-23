<script lang="ts">
	import { Pie } from 'svelte-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		ArcElement,
		RadialLinearScale
	} from 'chart.js';
	import type { DataSource } from '$lib/types/chart.js';
	import ChartDataLabels from 'chartjs-plugin-datalabels';

	export let dataSources: DataSource[];
	export let title: string;

	type Dataset = {
		data: number[];
		backgroundColor: string[];
	};
	type Labels = string[];

	const initialValue: { dataset: Dataset; labels: Labels } = {
		labels: [],
		dataset: { data: [], backgroundColor: [] }
	};

	ChartJS.register(Title, Tooltip, Legend, ArcElement, RadialLinearScale, ChartDataLabels);

	const { dataset, labels } = dataSources.reduce((prev, current) => {
		prev.labels.push(current.label);
		prev.dataset.data.push(current.value);
		prev.dataset.backgroundColor.push(current.color ?? '#d68fb8');

		return prev;
	}, initialValue);

	const data = { datasets: [dataset], labels };
</script>

<Pie
	{data}
	options={{
		responsive: true,
		plugins: {
			title: {
				display: true,
				text: title,
				font: {
					size: 24
				}
			}
		}
	}}
/>
