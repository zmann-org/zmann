<script>
	import {
		ContextMenu,
		ContextMenuDivider,
		ContextMenuGroup,
		ContextMenuOption
	} from 'carbon-components-svelte';
	import CopyFile from 'carbon-icons-svelte/lib/CopyFile.svelte';
	import Cut from 'carbon-icons-svelte/lib/Cut.svelte';
	import Renew from 'carbon-icons-svelte/lib/Renew.svelte';
	/**
	 * @type {string[] | undefined}
	 */
	let selectedIds = [];
</script>

<ContextMenu>
	<ContextMenuOption indented labelText="Copy" shortcutText="⌘C" icon={CopyFile} />
	<ContextMenuOption indented labelText="Cut" shortcutText="⌘X" icon={Cut} />
	<ContextMenuDivider />
	<ContextMenuOption indented labelText="Export as">
		<ContextMenuGroup labelText="Export options" bind:selectedIds>
			<ContextMenuOption id="csv" icon={Cut} indented labelText="CSV" />
			<ContextMenuOption id="png" icon={Cut} indented labelText="PNG" />
			<ContextMenuOption id="jpg" icon={Cut} indented labelText="JPG" />
		</ContextMenuGroup>
	</ContextMenuOption>
	<ContextMenuDivider />
	<ContextMenuOption labelText="Reload Page" indented icon={Renew} />
	<ContextMenuDivider />
	<ContextMenuGroup labelText="Style options">
		<ContextMenuOption id="0" labelText="Font smoothing" selected />
		<ContextMenuOption id="1" labelText="Reduce noise" />
		<ContextMenuOption id="2" labelText="Auto-sharpen" />
	</ContextMenuGroup>
	<ContextMenuDivider />
	<ContextMenuOption disabled indented kind="danger" labelText="Delete" />
</ContextMenu>
