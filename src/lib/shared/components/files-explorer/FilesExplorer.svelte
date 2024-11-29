<script lang="ts">
	import FolderIcon from '$lib/shared/icons/FolderIcon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import DistinguishedFileIcon from './DistinguishedFileIcon.svelte';
	import { type FSDirEntry } from './files.model';
	import FileRow from './FileRow.svelte';
	import { cls } from 'svelte-ux';

	let { selectedFiles = $bindable([]) }: { selectedFiles: FSDirEntry[] } = $props();

	function getCurrentDir(): string[] {
		const savedDir = localStorage.getItem(LS_KEY);

		if (!savedDir) {
			return ['/'];
		}

		const parts = savedDir.split('/').filter((part) => part !== '');

		if (savedDir.startsWith('/')) {
			return ['/', ...parts];
		}

		return parts;
	}

	const LS_KEY = 'bluestudio-files-dir';
	let files: FSDirEntry[] = $state([]);
	let currentDir = $state(getCurrentDir());
	const searchedDir = $derived(currentDir.join('/'));

	export function clearSelection() {
		selectedFiles = [];
	}

	async function fetchFiles(dir: string) {
		try {
			files = await invoke<FSDirEntry[]>('list_files', { dir }).then((files) =>
				files.sort((a, b) => a.name.localeCompare(b.name))
			);
		} catch (error) {
			console.error('Error fetching files:', error);
		}
	}

	function openFile(file: FSDirEntry) {
		if (file.is_dir) {
			currentDir.push(file.name);
			return;
		}

		if (selectedFiles.some((_file) => _file.path === file.path)) {
			selectedFiles = selectedFiles.filter((_file) => _file.path !== file.path);
		} else {
			selectedFiles.push(file);
		}
	}

	$effect(() => {
		fetchFiles(searchedDir);
		localStorage.setItem(LS_KEY, searchedDir);
	});

	onMount(() => {
		fetchFiles(searchedDir);
	});
</script>

<div class="flex flex-col max-h-full styled-scrollbar overflow-auto overscroll-none text-sm">
	<div
		class="font-semibold font-mono sticky top-0 bg-gray-900 w-full h-5 text-gray-400 text-xs px-2 py-0.5"
	>
		{searchedDir.replace('/', '') || '/'}
	</div>

	<table class="">
		<tbody>
			{#if currentDir.length > 1 || searchedDir.length > 1}
				<FileRow
					highlighted={false}
					onclick={() => {
						if (currentDir.length > 1) {
							currentDir.pop();
						}
					}}
				>
					<span class="px-2 w-full block">..</span>
				</FileRow>
			{/if}

			{#each files as file}
				<FileRow
					highlighted={selectedFiles.some((_file) => _file.path === file.path)}
					onclick={() => openFile(file)}
				>
					<div class="flex items-center gap-2 px-2">
						{#if file.is_dir}
							<FolderIcon class="w-4 h-4" />
						{:else}
							<DistinguishedFileIcon name={file.name} />
						{/if}
						{file.name}
					</div>
				</FileRow>
			{/each}
		</tbody>
	</table>
</div>
