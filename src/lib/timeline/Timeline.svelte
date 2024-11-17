<script lang="ts">
	const tracks: Track[] = $state([]);
</script>

{#each tracks as track}
	{#if track.type === 'video'}
		<video>
			{#each track.frames as frame}
				<img src={frame} />
			{/each}
		</video>
	{:else if track.type === 'audio'}
		<audio src={track.src} />
	{/if}
{/each}

<!-- const tracks = [
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
