<script lang="ts">
	import { cls } from 'svelte-ux';
	import FilesExplorer from '$lib/shared/components/files-explorer/FilesExplorer.svelte';
	import FileRow from '$lib/shared/components/files-explorer/FileRow.svelte';
	import DistinguishedFileIcon from '$lib/shared/components/files-explorer/DistinguishedFileIcon.svelte';
	import { type FileInfo } from '$lib/shared/components/files-explorer/FileInfo';

	let filesExplorer: FilesExplorer;
	let selectedFiles: FileInfo[] = $state([]);
	let importedFiles: FileInfo[] = $state([]);
</script>

<main class="w-full flex h-full">
	<div class="w-1/2 flex flex-col">
		<div class="flex-1 h-full overflow-auto">
			<FilesExplorer bind:this={filesExplorer} bind:selectedFiles />
		</div>

		<div
			class={cls('flex shrink-0 h-10 items-center justify-between px-3', {
				'bg-blue-700': selectedFiles.length > 0,
				'bg-gray-800': selectedFiles.length === 0
			})}
		>
			<button
				class={cls(
					'flex w-full h-full items-center gap-1 text-xs font-semibold font-mono',
					{
						'text-gray-400': selectedFiles.length === 0,
						'text-white': selectedFiles.length > 0
					}
				)}
				onclick={() => {
					importedFiles = importedFiles.concat(selectedFiles);
					filesExplorer.clearSelection();
				}}
			>
				{#if selectedFiles.length > 0}
					<span>Import {selectedFiles.length} selected files</span>
				{:else}
					<span>Select files to import</span>
				{/if}
			</button>
		</div>
	</div>

	<div class="w-1/2 border-l border-gray-800 h-full">
		<div class="flex-1 w-full h-full flex items-center justify-center">
			{#if importedFiles.length > 0}
				<div class="flex flex-col w-full h-full">
					{#each importedFiles as file}
						<FileRow highlighted={false} onclick={() => alert('selected file')}>
							<DistinguishedFileIcon name={file.name} />
							{file.name}
						</FileRow>
						<!-- <div class="flex items center gap-2">
							<span class="text-gray-400"></span>
							<button
								class="text-xs text-gray-400"
								onclick={() =>
									(importedFiles = importedFiles.filter((f) => f !== file))}
							>
								Remove
							</button>
						</div> -->
					{/each}
				</div>
			{:else}
				<p class="text-xl text-gray-500 font-light">No media imported</p>
			{/if}
		</div>
	</div>
</main>
