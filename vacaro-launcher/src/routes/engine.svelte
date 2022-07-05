<script>
	import { InlineLoading, RadioTile, TileGroup } from 'carbon-components-svelte';
</script>

<TileGroup legend="Service pricing tiers">
	<RadioTile value="0" checked
		>Lite plan <InlineLoading status="active" description="Submitting..." />
	</RadioTile>
	<RadioTile value="1"
		>Standard plan <InlineLoading status="finished" description="Success" />
	</RadioTile>
	<RadioTile value="2"
		>Plus plan <InlineLoading status="error" description="An error occurred" />
	</RadioTile>
</TileGroup>
