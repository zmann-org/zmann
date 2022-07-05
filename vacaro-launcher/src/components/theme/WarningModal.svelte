<script>
	import { Modal } from 'carbon-components-svelte';
	export let themeKey = 'g10';
	export let isDarkModeModal = false;
</script>

<Modal
	bind:open={isDarkModeModal}
	modalHeading="Theme Change"
	primaryButtonText="Confirm"
	secondaryButtonText="Cancel"
	on:click:button--secondary={() => (isDarkModeModal = false)}
	on:open
	on:close
	on:click:button--primary={() => (themeKey = 'g90')}
>
	<p>The dark theme is currently in development and not supported.</p>
</Modal>
