<script lang="ts">
	import { cls } from 'svelte-ux';
	import FilesExplorer from '$lib/shared/components/files-explorer/FilesExplorer.svelte';
	import FileRow from '$lib/shared/components/files-explorer/FileRow.svelte';
	import DistinguishedFileIcon from '$lib/shared/components/files-explorer/DistinguishedFileIcon.svelte';
	import { type FileInfo } from '$lib/shared/components/files-explorer/FileInfo';
	import { page } from '$app/stores';
	import { timelineState } from '$lib/shared/media/application/timeline.state.svelte';
	import { TimelineTrackType } from '$lib/shared/media/domain/timeline-track';
	import { invoke } from '@tauri-apps/api/core';

	const { projectId } = $page.params;
	const LS_KEY = `bluestudio-${projectId}-project-files`;

	function getProjectFiles() {
		const projectFiles = JSON.parse(localStorage.getItem(LS_KEY) || '[]');
		return projectFiles;
	}

	function addFilesToTimeline(files: FileInfo[]) {
		function process_video(file: FileInfo) {
			const track = {
				type: TimelineTrackType.VIDEO,
				start: 0,
				end: 0,
				frames: [file.path]
			};
		}
	}

	async function extractFramesFromVideo(videoUrl: string) {
		const frames = await invoke<string[]>('extract_frames_from_video', { videoUrl });
		return frames;
	}

	let filesExplorer: FilesExplorer;
	let filesSelectedForImport: FileInfo[] = $state([]);
	let selectedImportedFiles: FileInfo[] = $state([]);
	let projectFiles: FileInfo[] = $state(getProjectFiles());

	$effect(() => {
		localStorage.setItem(LS_KEY, JSON.stringify(projectFiles));
	});
</script>

<div class="w-full flex h-full select-none">
	<div class="w-1/2 flex flex-col">
		<div class="flex-1 h-full overflow-auto">
			<FilesExplorer bind:this={filesExplorer} bind:selectedFiles={filesSelectedForImport} />
		</div>

		<div
			class={cls('flex shrink-0 h-10 items-center justify-between px-3', {
				'bg-blue-700': filesSelectedForImport.length > 0,
				'bg-gray-800': filesSelectedForImport.length === 0
			})}
		>
			<button
				class={cls(
					'flex w-full h-full items-center gap-1 text-xs font-semibold font-mono',
					{
						'text-gray-400': filesSelectedForImport.length === 0,
						'text-white': filesSelectedForImport.length > 0
					}
				)}
				onclick={() => {
					projectFiles = projectFiles.concat(filesSelectedForImport);
					filesExplorer.clearSelection();
				}}
			>
				{#if filesSelectedForImport.length > 0}
					<span>Import {filesSelectedForImport.length} selected files</span>
				{:else}
					<span>Select files to import</span>
				{/if}
			</button>
		</div>
	</div>

	<div class="w-1/2 border-l border-gray-800 h-full">
		<div class="flex-1 w-full h-full flex items-center justify-center">
			{#if projectFiles.length > 0}
				<div class="flex flex-col w-full h-full">
					<div
						class="font-semibold font-mono sticky top-0 bg-gray-900 w-full h-5 text-gray-400 text-xs px-2"
					>
						Project files: {projectFiles.length}
					</div>

					{#each projectFiles as file}
						<FileRow
							highlighted={selectedImportedFiles.some(
								(_file) => _file.path === file.path
							)}
							onclick={() => {
								if (
									selectedImportedFiles.some((_file) => _file.path === file.path)
								) {
									selectedImportedFiles = selectedImportedFiles.filter(
										(_file) => _file.path !== file.path
									);
								} else {
									selectedImportedFiles.push(file);
								}
							}}
						>
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

					<div
						class={cls('flex shrink-0 h-10 items-center justify-between px-3', {
							'bg-blue-700': selectedImportedFiles.length > 0,
							'bg-gray-800': selectedImportedFiles.length === 0
						})}
					>
						<button
							class={cls(
								'flex w-full h-full items-center gap-1 text-xs font-semibold font-mono',
								{
									'text-gray-400': selectedImportedFiles.length === 0,
									'text-white': selectedImportedFiles.length > 0
								}
							)}
							onclick={() => {
								addFilesToTimeline(selectedImportedFiles);
							}}
						>
							{#if selectedImportedFiles.length > 0}
								<span>Add {selectedImportedFiles.length} selected files</span>
							{:else}
								<span>Select files to add</span>
							{/if}
						</button>
					</div>
				</div>
			{:else}
				<p class="text-xl text-gray-500 font-light">No media imported</p>
			{/if}
		</div>
	</div>
</div>
