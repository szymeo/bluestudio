<script lang="ts">
	import FolderIcon from '$lib/shared/icons/FolderIcon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import DistinguishedFileIcon from './DistinguishedFileIcon.svelte';
	import { type FileInfo } from './FileInfo';
	import FileRow from './FileRow.svelte';

	let { selectedFiles = $bindable([]) }: { selectedFiles: FileInfo[] } = $props();

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
	let files: FileInfo[] = $state([]);
	let currentDir = $state(getCurrentDir());
	const searchedDir = $derived(currentDir.join('/'));

	export function clearSelection() {
		selectedFiles = [];
	}

	async function fetchFiles(dir: string) {
		try {
			files = await invoke<FileInfo[]>('list_files', { dir }).then((files) =>
				files.sort((a, b) => a.name.localeCompare(b.name))
			);
		} catch (error) {
			console.error('Error fetching files:', error);
		}
	}

	function openFile(file: FileInfo) {
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
	<ul class="">
		<div
			class="font-semibold font-mono sticky top-0 bg-gray-900 w-full h-5 text-gray-400 text-xs px-2"
		>
			{searchedDir.replace('/', '') || '/'}
		</div>

		{#if currentDir.length > 1 || searchedDir.length > 1}
			<FileRow highlighted={false} onclick={() => currentDir.pop()}>..</FileRow>
		{/if}

		{#each files as file}
			<FileRow
				highlighted={selectedFiles.some((_file) => _file.path === file.path)}
				onclick={() => openFile(file)}
			>
				{#if file.is_dir}
					<FolderIcon class="w-4 h-4" />
				{:else}
					<DistinguishedFileIcon name={file.name} />
				{/if}
				{file.name}
			</FileRow>
		{/each}
	</ul>
</div>
