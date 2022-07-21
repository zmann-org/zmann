<script>
	import 'carbon-components-svelte/css/all.css'; // import carbon components system.

	import Header from '../components/UIShell/Header.svelte';
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
		HeaderUtilities
	} from 'carbon-components-svelte';

</script>

<Header
    company="test"
>
	<svelte:fragment slot="skip-to-content">
		<SkipToContent />
	</svelte:fragment>
	<HeaderNav>
		<HeaderNavItem href="/" text="Link 1" />
		<HeaderNavItem href="/" text="Link 2" />
		<HeaderNavItem href="/" text="Link 3" />
		<HeaderNavMenu text="Menu">
			<HeaderNavItem href="/" text="Link 1" />
			<HeaderNavItem href="/" text="Link 2" />
			<HeaderNavItem href="/" text="Link 3" />
		</HeaderNavMenu>
	</HeaderNav>
	<HeaderUtilities>
		<HeaderSearch placeholder="Search on Vacaro.org" />
	</HeaderUtilities>
</Header>

<Content>
	<slot />
</Content>
