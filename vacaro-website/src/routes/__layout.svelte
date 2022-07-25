<script>
	import 'carbon-components-svelte/css/all.css'; // import carbon components system.
	import { navigating } from '$app/stores'
	import Header from '../components/UIShell/Header.svelte';
	import HeaderNavItemHref from '../components/UIShell/HeaderNavItemHref.svelte';
	import {
		HeaderNav,
		HeaderNavItem,
		HeaderNavMenu,
		SideNav,
		SideNavItems,
		SideNavMenu,
		SideNavMenuItem,
		SideNavLink,
		SkipToContent,
		Content,
		HeaderSearch,
		HeaderUtilities,
		HeaderAction,
		HeaderPanelLinks,
		HeaderPanelDivider,
		HeaderPanelLink,
		HeaderGlobalAction,
		SideNavDivider,
		Theme,
		Loading
	} from 'carbon-components-svelte';

	import User from 'carbon-icons-svelte/lib/User.svelte';
	import MoonThemeSwitchIcon from 'carbon-icons-svelte/lib/Moon.svelte';
	import SunThemeSwitchIcon from 'carbon-icons-svelte/lib/Sun.svelte';

	let isOpen = false;

	let /**
		 * @type {import('carbon-components-svelte/types/Theme/Theme.svelte').CarbonTheme}
		 */ theme = 'g10';
	let isSideNavOpen = false;
</script>

<Theme bind:theme persist persistKey="__carbon-theme" />
<Header company="Vacaro Engine" persistentHamburgerMenu bind:isSideNavOpen>
	<svelte:fragment slot="skip-to-content">
		<SkipToContent />
	</svelte:fragment>
	<HeaderNav>
		<HeaderNavItemHref StartsWith="false" Href="/" Text="Let's Create" />
		<HeaderNavMenu text="Products & Solutions">
			<HeaderNavItem href="/" text="Link 1" />
			<HeaderNavItem href="/" text="Link 2" />
			<HeaderNavItem href="/" text="Link 3" />
		</HeaderNavMenu>
		<HeaderNavMenu text="Consulting & Services">
			<HeaderNavItem href="/" text="Link 1" />
			<HeaderNavItem href="/" text="Link 2" />
			<HeaderNavItem href="/" text="Link 3" />
		</HeaderNavMenu>
		<HeaderNavItem href="/test" text="Link 2" />
		<HeaderNavItem href="/" text="Link 3" />
	</HeaderNav>
	<HeaderUtilities>
		<HeaderSearch placeholder="Search on Vacaro.org" />
		{#if theme == "g90"}
			<HeaderGlobalAction
				on:click={() => {theme = "g10";}}
				aria-label="Toggle Theme"
				icon={MoonThemeSwitchIcon}
			/>
		{:else}
			<HeaderGlobalAction
				on:click={() => {theme = "g90";}}
				aria-label="Toggle Theme"
				icon={SunThemeSwitchIcon}
			/>
		{/if}
		<HeaderGlobalAction aria-label="Account" icon={User} />
		<HeaderAction bind:isOpen>
			<HeaderPanelLinks>
				<HeaderPanelDivider>Switcher subject 1</HeaderPanelDivider>
				<HeaderPanelLink>Switcher item 1</HeaderPanelLink>
				<HeaderPanelDivider>Switcher subject 2</HeaderPanelDivider>
				<HeaderPanelLink>Switcher item 1</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 2</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 3</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 4</HeaderPanelLink>
				<HeaderPanelLink>Switcher item 5</HeaderPanelLink>
			</HeaderPanelLinks>
		</HeaderAction>
	</HeaderUtilities>
</Header>

<SideNav bind:isOpen={isSideNavOpen}>
	<SideNavItems>
		<SideNavLink text="Link 1" />
		<SideNavLink text="Link 2" />
		<SideNavLink text="Link 3" />
		<SideNavMenu text="Menu">
			<SideNavMenuItem href="/" text="Link 1" />
			<SideNavMenuItem href="/" text="Link 2" />
			<SideNavMenuItem href="/" text="Link 3" />
		</SideNavMenu>
		<SideNavDivider />
		<SideNavLink text="Link 4" />
	</SideNavItems>
</SideNav>

<Content>
	{#if $navigating}
		<Loading/>
		<script>
			console.log('test');
		</script>
	{/if}
	<slot />
</Content>
