<script lang="ts">
	import { page } from '$app/stores';
	import DistinguishedFileIcon from '$lib/shared/components/files-explorer/DistinguishedFileIcon.svelte';
	import FileRow from '$lib/shared/components/files-explorer/FileRow.svelte';
	import {
		type FSDirEntry,
		type ProjectFile,
		type RawProjectFile
	} from '$lib/shared/components/files-explorer/files.model';
	import FilesImporter from '$lib/shared/components/files-explorer/FilesImporter.svelte';
	import ImportIcon from '$lib/shared/icons/ImportIcon.svelte';
	import { type Miliseconds } from '$lib/shared/types';
	import { formatTimeWithFrames } from '$lib/shared/utils';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { quintInOut } from 'svelte/easing';
	import { fly } from 'svelte/transition';

	const { projectId } = $page.params;

	function onImportFilesClicked() {
		// filesSelectedForImport.forEach((file) => {
		// 	if (!projectFiles.some((f) => f.path === file.path)) {
		// 		invoke<ProjectFile>('create_project_file', { projectId, path: file.path }).then(
		// 			(projectFile) => {
		// 				console.log('added file', projectFile);
		// 				projectFiles.push(projectFile);
		// 			}
		// 		);
		// 	}
		// });
	}

	let selectedImportedFiles: FSDirEntry[] = $state([]);
	let projectFiles: ProjectFile[] = $state([]);
	let importingMedia = $state(false);

	onMount(() => {
		console.log('getting project files for', projectId);

		invoke<RawProjectFile[]>('get_project_files', { projectId }).then((files) => {
			console.log('got project files', files);

			projectFiles = files.map((file) => ({
				...file.project_file,
				frames: file.frames,
				duration: file.duration as Miliseconds
			}));

			importingMedia = true;
		});
	});
</script>

<div class="flex-1 w-full h-full flex items-center justify-center relative">
	{#if importingMedia}
		<div
			class="absolute w-full h-full rounded-md overflow-hidden bg-gray-900"
			transition:fly={{ x: -30, duration: 550, easing: quintInOut }}
		>
			<FilesImporter
				onImport={(files) => {
					console.log('importing files', files);

					// filesSelectedForImport = files;
					// onImportFilesClicked();
					// importingMedia = false;
				}}
				onCancel={() => {
					importingMedia = false;
				}}
			/>
		</div>
	{/if}

	{#if projectFiles.length > 0}
		<div class="w-full h-full">
			<table class="table-fixed w-full">
				<thead class="border-b border-b-gray-700/80">
					<tr>
						<th class="text-left text-gray-100 text-xs px-3 py-2">Name</th>
						<th class="text-left text-gray-100 text-xs px-3 py-2">Frames</th>
						<th class="text-left text-gray-100 text-xs px-3 py-2">Duration</th>
						<th class="text-left text-gray-100 text-xs px-3 py-2">FPS</th>
						<th class="text-right text-gray-100 px-1">
							<button
								class="bg-gray-600/70 text-white px-2 text-xs py-0.5 rounded-md flex items-center ml-auto"
								onclick={() => {
									importingMedia = true;
								}}
							>
								Import more

								<ImportIcon class="w-3 h-3 ml-1" stroke="stroke-white" />
							</button>
						</th>
					</tr>
				</thead>

				<tbody>
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
							<td class="px-3 py-1">
								<div class="flex items-center gap-1.5">
									<DistinguishedFileIcon name={file.name} />
									{file.name}
								</div>
							</td>

							<td class="px-3 py-1">
								{file.frames.length}
							</td>

							<td class="px-3 py-1">
								{formatTimeWithFrames(file.duration, file.frames.length)}
							</td>

							<td class="px-3 py-1">
								{Number(file.frames.length / (file.duration / 1000)).toFixed(2)}
							</td>

							<td class="pr-4 text-right text-gray-400 group-hover:text-gray-300">
								<button
									class="text-xs hover:text-red-600 uppercase font-semibold tracking-wide"
									onclick={() => {
										projectFiles = projectFiles.filter((f) => f !== file);
									}}
								>
									Remove
								</button>
							</td>
						</FileRow>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<p class="text-xl text-gray-500 font-light">No media imported</p>
	{/if}
</div>
