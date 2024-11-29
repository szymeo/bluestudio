<script lang="ts">
	import { cls } from 'svelte-ux';
	import FilesExplorer from './FilesExplorer.svelte';
	import { type FSDirEntry } from './files.model';

	let filesSelectedForImport: FSDirEntry[] = $state([]);

	const {
		onImport,
		onCancel
	}: {
		onImport: (v: FSDirEntry[]) => void;
		onCancel: () => void;
	} = $props();
</script>

<div class="flex flex-col overflow-auto h-full">
	<div class="flex-1 h-full overflow-auto">
		<FilesExplorer bind:selectedFiles={filesSelectedForImport} />
	</div>

	<div
		class={cls('flex shrink-0 h-10 items-center justify-between gap-3', {
			'bg-blue-700': filesSelectedForImport.length > 0,
			'bg-gray-800': filesSelectedForImport.length === 0
		})}
	>
		<button
			class={cls(
				'flex w-full h-full items-center gap-1 text-xs font-semibold font-mono px-3',
				{
					'text-gray-400': filesSelectedForImport.length === 0,
					'text-white': filesSelectedForImport.length > 0
				}
			)}
			onclick={() => onImport(filesSelectedForImport)}
		>
			{#if filesSelectedForImport.length > 0}
				<span>Import {filesSelectedForImport.length} selected files</span>
			{:else}
				<span>Select files to import</span>
			{/if}
		</button>

		<button
			class={cls(
				'flex items-center gap-1 text-xs font-semibold font-mono text-white px-2 py-0.5 rounded-md mr-3 bg-red-700'
			)}
			onclick={onCancel}
		>
			Cancel
		</button>
	</div>
</div>
