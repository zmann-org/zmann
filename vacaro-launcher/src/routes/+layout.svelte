<script lang="ts">
	import 'carbon-components-svelte/css/all.css';

	import {
		Header,
		HeaderNav,
		HeaderNavItem,
		HeaderNavMenu,
		SkipToContent,
		Content,
		HeaderUtilities,
		HeaderAction,
		HeaderGlobalAction,
		HeaderPanelLinks,
		HeaderPanelDivider,
		HeaderPanelLink,
		SideNav,
		SideNavItems,
		SideNavMenu,
		SideNavMenuItem,
		SideNavLink,
		SideNavDivider,
		Theme
	} from 'carbon-components-svelte';
	import { onMount } from 'svelte';
	import { appWindow } from '@tauri-apps/api/window';
	import { goto } from '$app/navigation';

	import WindowMinimizeButton from '../components/icon/WindowMinimizeButton.svelte';
	import WindowCloseButton from '../components/icon/WindowCloseButton.svelte';
	import WindowMaximizeButton from '../components/icon/WindowMaximizeButton.svelte';
	import WindowRestoreButton from '../components/icon/WindowRestoreButton.svelte';
	import WindowMenuIconButton from '../components/icon/WindowMenuIconButton.svelte';

	import Settings from 'carbon-icons-svelte/lib/Settings.svelte';
	import MoonThemeSwitchIcon from 'carbon-icons-svelte/lib/Moon.svelte';
	import SunThemeSwitchIcon from 'carbon-icons-svelte/lib/Sun.svelte';
	import LogoGithub from 'carbon-icons-svelte/lib/LogoGithub.svelte';
	import LogoTwitter from 'carbon-icons-svelte/lib/LogoTwitter.svelte';
	import LogoDiscord from 'carbon-icons-svelte/lib/LogoDiscord.svelte';
	import LogoYoutube from 'carbon-icons-svelte/lib/LogoYoutube.svelte';
	import ApplicationWeb from 'carbon-icons-svelte/lib/ApplicationWeb.svelte';
	import WatsonHealthAiResultsUrgent from 'carbon-icons-svelte/lib/WatsonHealthAiResultsUrgent.svelte';
	import InformationSquare from 'carbon-icons-svelte/lib/InformationSquare.svelte';
	import Share from 'carbon-icons-svelte/lib/Share.svelte';
	import HeaderNavItemHref from '../components/navigation/HeaderNavItemHref.svelte';
	import HeaderUtilitiesDrag from '../components/navigation/HeaderUtilitiesDrag.svelte';
	import WarningModal from '../components/theme/WarningModal.svelte';
	import ContextMenuLayout from '../components/ContextMenuLayout.svelte';
	import HeaderName from '../components/navigation/HeaderName.svelte';

	let isOpen = false;
	let isSideNavOpen = false;
	let isDarkModeModal = false;
	let isMaximized = false;

	let /**
		 * @type {import('carbon-components-svelte/types/Theme/Theme.svelte').CarbonTheme}
		 */ theme = 'g10';

	function closeApp() {
		appWindow.close();
	}
	function minimizeApp() {
		appWindow.minimize();
	}
	function toggleApp() {
		// appWindow.isMaximized().then((value) => {
		// 	if (isMaximized == true) {
		// 		if (value == true) alert('test1');
		// 		else alert('test2');
		// 	} else {
		// 		if (value == true) alert('test3');
		// 		else alert('test4');
		// 	}
		// });
		appWindow.toggleMaximize();
		isMaximized = !isMaximized;
	}

	appWindow.onResized(() => {
		appWindow.isMaximized().then((e) => {
			if (e == true) isMaximized = true;
			else isMaximized = false;
		});
	});

	function routeToPage(route: string, replaceState: boolean) {
		goto(`/${route}`, { replaceState });
	}
</script>

<div>
	<Theme bind:theme persist persistKey="__carbon-theme" />
	<HeaderName
		data-tauri-drag-region
		class="titlebar"
		company="Vacaro"
		platformName="Launcher"
		bind:isSideNavOpen
		iconMenu={WindowMenuIconButton}
		persistentHamburgerMenu={false}
	>
		<svelte:fragment slot="skip-to-content">
			<SkipToContent />
		</svelte:fragment>

		<HeaderNav data-tauri-drag-region>
			<HeaderNavItemHref Href="/" Text="Home" StartsWith="false" />
			<HeaderNavItemHref Href="/engine" Text="Engine" StartsWith="false" />
			<HeaderNavItemHref Href="/assets" Text="Assets" StartsWith="true" />
			<HeaderNavMenu text="Documentation">
				<HeaderNavItemHref
					Href="/docs/getting-started"
					Text="Getting Started"
					StartsWith="true"
				/>
				<HeaderNavItemHref
					Href="/docs/account-managment"
					Text="Account Management"
					StartsWith="false"
				/>
				<HeaderNavItemHref
					Href="/docs/self-hosting"
					Text="Self Hosting"
					StartsWith="false"
				/>
			</HeaderNavMenu>
		</HeaderNav>
		<HeaderUtilitiesDrag>
			{#if theme == 'g90'}
				<HeaderGlobalAction
					on:click={() => {
						theme = 'g10';
					}}
					aria-label="Toggle Theme"
					icon={MoonThemeSwitchIcon}
				/>
			{:else}
				<HeaderGlobalAction
					on:click={() => {
						theme = 'g90';
					}}
					aria-label="Toggle Theme"
					icon={SunThemeSwitchIcon}
				/>
			{/if}
			<HeaderGlobalAction
				aria-label="Settings"
				on:click={() => routeToPage('settings', false)}
				icon={Settings}
			/>
			<HeaderAction bind:isOpen data-tauri-drag-region>
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
		</HeaderUtilitiesDrag>
		<div
			style="width=100%; margin-left: 20px; margin-right: 1.5px; height=100%"
			data-tauri-drag-region={true}
		>
			<HeaderGlobalAction
				aria-label="Minimize"
				on:click={minimizeApp}
				class="utilbutton"
				icon={WindowMinimizeButton}
			/>
			{#if isMaximized == false}
				<HeaderGlobalAction
					aria-label="Maximize"
					on:click={toggleApp}
					class="utilbutton"
					icon={WindowMaximizeButton}
				/>
			{:else}
				<HeaderGlobalAction
					aria-label="Restore"
					on:click={toggleApp}
					class="utilbutton"
					icon={WindowRestoreButton}
				/>
			{/if}
			<HeaderGlobalAction
				on:click={closeApp}
				aria-label="Close"
				class="closebutton"
				icon={WindowCloseButton}
			/>
		</div>
	</HeaderName>

	<SideNav rail bind:isOpen={isSideNavOpen}>
		<!-- style={themeBackground} -->
		<SideNavItems>
			<SideNavLink
				icon={LogoGithub}
				text="Visit GitHub"
				href="https://github.com/vacaro"
				target="_blank"
			/>
			<!-- isSelected -->
			<SideNavLink
				icon={LogoDiscord}
				text="Join the Discord Server"
				href="https://discord.gg/M5SncHjSX5"
				target="_blank"
			/>
			<SideNavMenu icon={Share} text="Social Media">
				<!-- <SideNavMenuItem href="/" text="Link 1" /> -->
				<SideNavLink href="/" icon={LogoTwitter} text="Open Twitter" />
				<SideNavLink href="/" icon={LogoYoutube} text="Open YT Channel" />
				<SideNavLink href="/" icon={ApplicationWeb} text="Open the Website" />
			</SideNavMenu>
			<SideNavDivider />
			<SideNavLink
				icon={WatsonHealthAiResultsUrgent}
				text="Report an Issue"
				href="https://github.com/Vacaro/Vacaro/issues/new"
				target="_blank"
			/><SideNavLink
				icon={InformationSquare}
				text="Open Docuemtation"
				href="https://github.com/Vacaro/Vacaro/issues/new"
				target="_blank"
			/>
		</SideNavItems>
	</SideNav>

	<ContextMenuLayout />

	<Content>
		<WarningModal bind:isDarkModeModal bind:theme />
		<slot />
		<dds-back-to-top />
	</Content>

	<style>
		.titlebar {
			height: 32px;
			user-select: none;
		}
		.titlebar-button {
			display: inline-flex;
			justify-content: center;
			align-items: center;
			width: 30px;
			height: 30px;
		}
		.titlebar-button:hover {
			background: #5bbec3;
		}
		.closebutton {
			/* height: 30px; */
			top: 0;
			color: white;
			fill: transparent !important;
			/* margin-bottom: 17px; */
			margin-right: -1px;
		}
		.closebutton:hover {
			background-color: #e81123;
		}
		.closebutton:focus {
			border: 1px solid solid var(--cds-ui-background);
		}
		.utilbutton {
			/* height: 30px; */
			top: 0;
			/* margin-bottom: 17px; */
			color: white;
			fill: transparent !important;
		}
		.utilbutton:hover {
			background-color: #616161;
		}
		.utilbutton:focus {
			border: 1px solid solid var(--cds-ui-background);
		}
	</style>
</div>
