<script>
	import { goto } from '$app/navigation';
	import ProjectFilesManager from '$lib/media/ProjectFilesManager.svelte';
	import Player from '$lib/player/Player.svelte';
	import ArrowLeft from '$lib/shared/icons/ArrowLeftIcon.svelte';
	import Timeline from '$lib/timeline/Timeline.svelte';
	import { page } from '$app/stores';
	const { projectId } = $page.params;
</script>

<main class="w-full flex flex-col pt-7 h-full text-white select-none">
	<div
		data-tauri-drag-region="true"
		class="draggable absolute top-0 h-7 w-full px-20 flex items-center justify-center bg-gray-800"
	>
		<button
			onclick={() => goto('/')}
			class="absolute left-20 flex items-center text-xs font-mono gap-1 text-gray-400"
		>
			<ArrowLeft stroke="stroke-gray-400" class="w-4 h-4" />
			<span>back</span>
		</button>

		<span class="block text-xs text-gray-400 font-bold font-sans pointer-events-none">
			Untitled Project ({projectId})
		</span>
	</div>

	<div class="w-full h-full flex flex-col px-3 py-3 gap-2">
		<div class="flex items-center justify-center rounded-md select-none h-1/2 gap-2">
			<div class="bg-gray-900/90 border border-gray-800/50 h-full rounded-md">
				<ProjectFilesManager />
			</div>

			<div class="bg-gray-900/90 border border-gray-800/50 h-full w-full rounded-md">
				<Player />
			</div>
		</div>

		<div
			class="col-span-2 bg-gray-900/90 border border-gray-800/50 rounded-md h-full select-none"
		>
			<Timeline />
		</div>
	</div>
</main>

<!-- const framesMap = {
    klatka1: 'base64klatki1',
    klatka2: 'base64klatki2',
    klatka3: 'base64klatki3',
}

const audioMap = {
    audio1: 'base64audio1',
    audio2: 'base64audio2',
}

const timeline = [
    {
        type: 'video',
        start: timecode1,
        end: timecode2,
        frames: [
            klatka1,
            klatka2,
            klatka3,
        ]
    }, {
        type: 'audio',
        start: timecode1,
        end: timecode2,
        src: audio1,
    }
] -->
