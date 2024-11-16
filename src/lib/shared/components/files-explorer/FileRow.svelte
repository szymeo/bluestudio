<script lang="ts">
	import { cls } from 'svelte-ux';
	import type { Snippet } from 'svelte';

	const {
		highlighted,
		onclick,
		children
	}: { highlighted: boolean; onclick: () => void; children: Snippet } = $props();
</script>

<li
	class={cls('p-0.5 px-1.5 cursor-pointer flex items-center gap-2 select-none', {
		'bg-blue-800/30 text-white': highlighted,
		'hover:bg-gray-800 text-gray-400 hover:text-white': !highlighted
	})}
	{onclick}
>
	{@render children()}
</li>
