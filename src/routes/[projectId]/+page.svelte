<script>
	import ArrowLeft from '$lib/shared/icons/ArrowLeft.svelte';
</script>

<main class="w-full flex flex-col h-full text-white">
	<div
		data-tauri-drag-region="true"
		class="draggable h-7 w-full px-20 flex items-center justify-center bg-gray-800"
	>
		<button class="absolute left-20 flex items-center text-xs font-mono gap-1 text-gray-400">
			<ArrowLeft stroke="stroke-gray-400" class="w-4 h-4" />
			<span>back</span>
		</button>

		<span class="block text-xs text-gray-400 font-bold font-sans">Project 1</span>
	</div>

	<div class="w-full h-full flex flex-col px-3 py-3 gap-2">
		<div class="w-full flex-1 flex items-center justify-center gap-2">
			<div class="bg-gray-900 rounded-md w-full h-full"></div>
			<div class="bg-gray-900 rounded-md w-full h-full"></div>
		</div>

		<div class="w-full flex-1 bg-gray-900 rounded-md"></div>
	</div>
</main>
