<script lang="ts">
	import { cls } from 'svelte-ux';
	import FilesExplorer from '$lib/shared/components/files-explorer/FilesExplorer.svelte';
	import FileRow from '$lib/shared/components/files-explorer/FileRow.svelte';
	import DistinguishedFileIcon from '$lib/shared/components/files-explorer/DistinguishedFileIcon.svelte';
	import {
		type FSDirEntry,
		type ProjectFile,
		type RawProjectFile
	} from '$lib/shared/components/files-explorer/files.model';
	import { type Miliseconds } from '$lib/shared/types';
	import { page } from '$app/stores';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	const { projectId } = $page.params;

	function onImportFilesClicked() {
		filesSelectedForImport.forEach((file) => {
			if (!projectFiles.some((f) => f.path === file.path)) {
				invoke<ProjectFile>('create_project_file', { projectId, path: file.path }).then(
					(projectFile) => {
						console.log('added file', projectFile);

						projectFiles.push(projectFile);
					}
				);
			}
		});
	}

	let filesSelectedForImport: FSDirEntry[] = $state([]);
	let selectedImportedFiles: FSDirEntry[] = $state([]);
	let projectFiles: ProjectFile[] = $state([]);

	onMount(() => {
		console.log('getting project files for', projectId);

		invoke<RawProjectFile[]>('get_project_files', { projectId }).then((files) => {
			console.log('got project files', files);

			projectFiles = files.map((file) => ({
				...file.project_file,
				frames: file.frames,
				duration: file.duration as Miliseconds
			}));
		});
	});

	function formatTimeWithFrames(totalSeconds: number, fps: number): string {
		const hours: number = Math.floor(totalSeconds / 3600);
		const minutes: number = Math.floor((totalSeconds % 3600) / 60);
		const seconds: number = Math.floor(totalSeconds % 60);

		const frames: number = Math.floor((totalSeconds % 1) * fps);

		const pad = (num: number, size: number): string => num.toString().padStart(size, '0');

		return `${pad(hours, 2)}:${pad(minutes, 2)}:${pad(seconds, 2)}:${pad(frames, 2)}`;
	}
</script>

<div class="flex w-full h-full select-none">
	<div class="w-1/2 flex flex-col">
		<div class="flex-1 h-full overflow-auto">
			<FilesExplorer bind:selectedFiles={filesSelectedForImport} />
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
				onclick={onImportFilesClicked}
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
									// selectedImportedFiles.push(file);
								}
							}}
						>
							<div class="flex gap-1.5">
								<DistinguishedFileIcon name={file.name} />
								{file.name}
							</div>
							|
							<div>
								{file.frames.length} frames
							</div>
							|
							<div>
								{formatTimeWithFrames(file.duration / 1000, file.frames.length)} duration
							</div>
							|
							<div>
								{file.frames.length / (file.duration / 1000)} fps
							</div>
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
								// addFilesToTimeline(selectedImportedFiles);
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
