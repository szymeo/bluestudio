<script lang="ts">
	import FolderIcon from '$lib/shared/icons/FolderIcon.svelte';
	import LogoIcon from '$lib/shared/icons/LogoIcon.svelte';
	import VideoIcon from '$lib/shared/icons/PlusIcon.svelte';
	import VideoLibraryIcon from '$lib/shared/icons/VideoLibraryIcon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	type Project = {
		id: string;
		name: string;
	};

	function getProjects(): Promise<Project[]> {
		return invoke<Project[]>('get_projects');
	}

	let projects: Project[] = $state([]);

	function createProject() {}

	onMount(async () => {
		projects = await getProjects();
	});
</script>

<main class="w-full flex flex-col h-full text-white">
	<div
		data-tauri-drag-region="true"
		class="absolute top-0 draggable h-7 w-full px-20 flex items-center justify-center text-sm font-semibold"
	></div>

	<div class="w-full h-full flex">
		<div class="w-1/3 flex flex-col h-full bg-gray-900 py-20 px-4">
			<h1 class="text-4xl font-mono justify-start w-full flex items-center h-fit">
				<LogoIcon class="w-12 h-12 mr-1" />
				BlueStudio
			</h1>

			<span class="text-gray-300 pl-14 mt-2">
				Your movie editor is here!
				<br />
				To get started, create a new project or open an existing one.
			</span>
		</div>

		<div class="w-2/3 h-full bg-gray-800 p-6">
			<h2 class="text-2xl font-semibold flex items-center gap-1.5">
				Projects

				<VideoLibraryIcon class="w-7 h-7" stroke="stroke-gray-100" />
			</h2>
			<ul class="mt-6">
				{#each projects as project}
					<li class="flex items-center gap-2 cursor-pointer">
						<FolderIcon class="w-4 h-4" />
						<span>{project.name}</span>
					</li>
				{/each}

				<li
					class="flex items-center gap-2 cursor-pointer bg-blue-700 font-semibold flex-col w-fit p-4 rounded-md"
					onclick={() => {}}
				>
					<VideoIcon class="w-7 h-7" stroke="stroke-white" />
					<span>New Project</span>
				</li>
			</ul>
		</div>
	</div>
</main>
