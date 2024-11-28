<script lang="ts">
	import AviFileIcon from '$lib/shared/icons/file-types/AviFileIcon.svelte';
	import JpgFileIcon from '$lib/shared/icons/file-types/JpgFileIcon.svelte';
	import Mp3FileIcon from '$lib/shared/icons/file-types/Mp3FileIcon.svelte';
	import Mp4FileIcon from '$lib/shared/icons/file-types/Mp4FileIcon.svelte';
	import PngFileIcon from '$lib/shared/icons/file-types/PngFileIcon.svelte';
	import SvgFileIcon from '$lib/shared/icons/file-types/SvgFileIcon.svelte';
	import WmvFileIcon from '$lib/shared/icons/file-types/WmvFileIcon.svelte';
	import ZipFileIcon from '$lib/shared/icons/file-types/ZipFileIcon.svelte';
	import FileIcon from '$lib/shared/icons/FileIcon.svelte';

	const { name } = $props();

	const fileType = name?.split('.').pop();
</script>

{#if fileType}
	{#if fileType === 'jpg'}
		<JpgFileIcon class="w-4 h-4" />
	{:else if fileType === 'mp3'}
		<Mp3FileIcon class="w-4 h-4" />
	{:else if fileType === 'png'}
		<PngFileIcon class="w-4 h-4" />
	{:else if fileType === 'wmv'}
		<WmvFileIcon class="w-4 h-4" />
	{:else if fileType === 'avi'}
		<AviFileIcon class="w-4 h-4" />
	{:else if fileType === 'mp4'}
		<Mp4FileIcon class="w-4 h-4" />
	{:else if fileType === 'svg'}
		<SvgFileIcon class="w-4 h-4" />
	{:else if fileType === 'zip'}
		<ZipFileIcon class="w-4 h-4" />
	{:else}
		<FileIcon class="w-4 h-4" />
	{/if}
{/if}
