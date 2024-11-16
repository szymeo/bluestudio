<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	let name = $state('');
	let greetMsg = $state('');

	async function greet(event: Event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke('greet', { name });
	}
</script>

<main class="w-full flex flex-col h-full text-white">
	<div
		data-tauri-drag-region="true"
		class="draggable h-7 w-full px-20 flex items-center justify-between bg-gray-800"
	></div>

	<a href="/1">Project 1</a>
</main>
