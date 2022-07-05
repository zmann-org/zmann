<script>
	import { SkeletonPlaceholder } from 'carbon-components-svelte';
	let showFrame = 'hidden';

	export /**
	 * @type {string}
	 */
	let url;
	export /**
	 * @type {string}
	 */
	let title;
</script>

{#if showFrame == 'hidden'}
	<SkeletonPlaceholder style="height: 100%; width: 100%;" />
{/if}

<iframe
	width="100%"
	height="100%"
	src={url}
	{title}
	frameborder="0"
	allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
	allowfullscreen
	style="visibility: {showFrame};"
	on:load={() => (showFrame = 'visible')}
/>
