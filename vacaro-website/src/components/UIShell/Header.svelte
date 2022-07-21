<script>
	/** Set to `false` to hide the side nav by default */
	export let expandedByDefault = true;

	/** Set to `true` to open the side nav */
	export let isSideNavOpen = false;

	/**
	 * Specify the ARIA label for the header
	 * @type {string}
	 */
	export let uiShellAriaLabel = undefined;

	/**
	 * Specify the `href` attribute
	 * @type {string}
	 */
	export let href = undefined;

	/**
	 * Specify the company name
	 * @type {string}
	 */
	export let company = undefined;

	/** Set to `true` to persist the hamburger menu */
	export let persistentHamburgerMenu = false;

	/**
	 * The window width (px) at which the SideNav is expanded and the hamburger menu is hidden
	 * 1056 represents the "large" breakpoint in pixels from the Carbon Design System:
	 * small: 320
	 * medium: 672
	 * large: 1056
	 * x-large: 1312
	 * max: 1584
	 */
	export let expansionBreakpoint = 1056;

	/** Obtain a reference to the HTML anchor element */
	export let ref = null;

	/**
	 * Specify the icon to render for the closed state.
	 * Defaults to `<Menu size={20} />`
	 * @type {typeof import("svelte").SvelteComponent}
	 */
	export let iconMenu = Menu;

	/**
	 * Specify the icon to render for the opened state.
	 * Defaults to `<Close size={20} />`
	 * @type {typeof import("svelte").SvelteComponent}
	 */
	export let iconClose = Close;

	import Close from 'carbon-icons-svelte/lib/Close.svelte';
	import Menu from 'carbon-icons-svelte/lib/Menu.svelte';
	import { shouldRenderHamburgerMenu } from 'carbon-components-svelte/src/UIShell/navStore.js';
	import HamburgerMenu from 'carbon-components-svelte/src/UIShell/HamburgerMenu.svelte';

	let winWidth = undefined;

	$: isSideNavOpen =
		expandedByDefault && winWidth >= expansionBreakpoint && !persistentHamburgerMenu;
	$: ariaLabel = company ? `${company} ` : '' + (uiShellAriaLabel || $$props['aria-label']);
</script>

<svelte:window bind:innerWidth={winWidth} />

<header aria-label={ariaLabel} class:bx--header={true}>
	<div class="bx--header__nav-container">
		<slot name="skip-to-content" />

		{#if ($shouldRenderHamburgerMenu && winWidth < expansionBreakpoint) || persistentHamburgerMenu}
			<HamburgerMenu bind:isOpen={isSideNavOpen} {iconClose} {iconMenu} />
		{/if}
		<a {href} class:bx--header__name={true} bind:this={ref} {...$$restProps} on:click>
			<span class:bx--header__name--prefix={true}>{company}&nbsp;</span>
		</a>
		<slot />
	</div>
</header>

<style>
	.bx--header__nav-container {
		height: 100%;
		overflow-x: visible;
		position: relative;
		display: flex;
		flex: 1;
		transition-timing-function: cubic-bezier(0.2, 0, 0.38, 0.9);
		border: none;
		max-width: 99rem;
		margin-left: auto;
		margin-right: auto;
		margin-bottom: 1px;
		position: relative;
		z-index: 5999;
		background-color: #161616;
	}
</style>
