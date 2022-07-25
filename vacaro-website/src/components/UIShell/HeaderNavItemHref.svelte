<script>
	import { HeaderNavItem } from 'carbon-components-svelte';
	import { page } from '$app/stores';
	export /**
	 * @type {string}
	 */
	let Href;
	export /**
	 * @type {string}
	 */
	let Text;
	export /**
	 * @type {string}
	 */
	let StartsWith;
</script>

{#if Href == $page.url.pathname}
	<HeaderNavItem href={Href} text={Text} isSelected />
{:else if StartsWith == 'true'}
	{#if $page.url.pathname.startsWith(Href)}
		<HeaderNavItem href={Href} text={Text} isSelected />
	{:else}
		<HeaderNavItem href={Href} text={Text} />
	{/if}
{:else}
	<HeaderNavItem href={Href} text={Text} />
{/if}
