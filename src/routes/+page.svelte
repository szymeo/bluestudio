<script lang="ts">
	import FolderIcon from '$lib/shared/icons/FolderIcon.svelte';
	import LogoIcon from '$lib/shared/icons/LogoIcon.svelte';
	import VideoIcon from '$lib/shared/icons/PlusIcon.svelte';
	import VideoLibraryIcon from '$lib/shared/icons/VideoLibraryIcon.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import dayjs from 'dayjs';

	type Project = {
		id: string;
		name: string;
		createdAt: string;
		updatedAt: string;
	};

	function getProjects(): Promise<Project[]> {
		return invoke<Project[]>('get_projects');
	}

	let projects: Project[] = $state([]);

	async function createProject() {
		alert('Creating a new project');
		const r = await invoke<Project>('create_project', { name: 'New Project' });
		console.log(r);
	}

	onMount(async () => {
		projects = await getProjects();
	});

	$inspect(projects);
</script>

<main class="w-full flex flex-col h-full text-white">
	<div
		data-tauri-drag-region="true"
		class="absolute top-0 draggable h-7 w-full px-20 flex items-center justify-center text-sm font-semibold"
	></div>

	<div class="w-full h-full flex">
		<div class="w-1/3 flex flex-col h-full bg-gray-900 py-20 px-8">
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

		<div class="divider"></div>

		<div class="w-2/3 h-full bg-gray-900 px-20 py-12">
			<h2 class="text-2xl font-semibold flex items-center gap-2">
				Projects

				<!-- <VideoLibraryIcon class="w-6 h-6" stroke="stroke-gray-100" /> -->
			</h2>

			<div class="mt-6">
				{#each projects as project, index}
					<li
						onclick={() => goto(`/${project.id}`)}
						class="flex hover:bg-gray-300/10 w-full rounded-xs py-1 items-center cursor-pointer gap-1.5"
					>
						<LogoIcon class="w-5 h-5 mr-1" />
						<span class="text-sm">{project.name}</span>
						<span class="text-xs text-gray-400 ml-2">
							{dayjs(new Date(project.createdAt)).format('DD MMM YY HH:mm')}
						</span>
					</li>
				{/each}

				<li
					class="flex border-gray-100/20 mt-4 pt-2 transition-all pl-2 border-t w-full py-1 items-center cursor-pointer gap-1.5"
					onclick={createProject}
				>
					<!-- <VideoIcon class="w-7 h-7" stroke="stroke-white" /> -->
					<span>+ Create Project</span>
				</li>
			</div>
		</div>
	</div>
</main>

<style lang="scss">
	@keyframes gradientAnimation {
		0% {
			background-position: 0% 0%;
		}
		50% {
			background-position: 0% -100%;
		}
		100% {
			background-position: 0% -200%;
		}
	}

	.divider {
		width: 1px;
		background: linear-gradient(to bottom, #ef587a, #614da3, #db8c21);
		background-size: 100% 200%;
		position: relative;
		opacity: 0.9;
		animation: gradientAnimation 5s ease infinite;

		&::after {
			content: '';
			position: absolute;
			filter: blur(20px);
			opacity: 0.25;
			background: linear-gradient(to bottom, #ef587a, #614da3, #db8c21);
			background-size: 100% 200%;
			top: 0;
			left: 100%;
			width: 10px; // Adjust the width of the shadow
			height: 100%;
			animation: gradientAnimation 5s ease infinite;
		}
	}
</style>
