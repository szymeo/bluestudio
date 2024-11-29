<script lang="ts">
	import { cls } from 'svelte-ux';
	import type { Snippet } from 'svelte';

	const {
		highlighted,
		onclick,
		children,
		class: className = ''
	}: { highlighted: boolean; onclick: () => void; children: Snippet; class?: string } = $props();
</script>

<tr
	class={cls(
		'p-0.5 px-1.5 group cursor-pointer border-b last-of-type:border-none border-gray-800/50 gap-2 select-none',
		className,
		{
			'bg-blue-900 text-white': highlighted,
			'hover:bg-gray-800/50 text-gray-100 hover:text-white': !highlighted
		}
	)}
	{onclick}
>
	{@render children()}
</tr>
