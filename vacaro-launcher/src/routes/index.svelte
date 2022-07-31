<script>
	import {
		Grid,
		Row,
		Column,
		Tile,
		AspectRatio,
		ButtonSet,
		Dropdown,
		Button
	} from 'carbon-components-svelte';
	import Launch from 'carbon-icons-svelte/lib/Launch.svelte';

	import GlassPanel from '../components/home/GlassPanel.svelte';
	import FAQ from '../components/home/FAQ.svelte';
	import YT from '../components/home/YT.svelte';
</script>

<Tile style="height: 512px; background-color: transparent;">
	<dds-leadspace
		size="medium"
		style="height: 100% !important; width: 100% !important; left: 0 !important; right: 0% !important; margin-left: 0vw !important; margin-right: 0vw !important; margin-top: -30px;"
		alt="Image alt text"
		default-src="1e7618ebe43eb5c8ce512709495c42fd.jpg"
		data-autoid="dds--leadspace"
		gradient-style-scheme="with-gradient"
		type="left"
	>
		<dds-leadspace-heading
			role="heading"
			aria-level="1"
			data-autoid="dds--leadspace-heading"
			slot="heading"><!---->Get Started<!----></dds-leadspace-heading
		>
		Create, Manage and Build projects.
		<dds-button-group
			slot="action"
			data-autoid="dds--button-group"
			role="list"
			style="--dds--button-group--item-count: 2;"
		>
			<!---->
			<dds-button-group-item
				aria-label=""
				href="https://example.com"
				data-autoid="dds--button-group-item"
				role="listitem"
				icon-layout=""
				kind="tertiary"
				><!---->Manage Projects<!----><svg
					focusable="false"
					preserveAspectRatio="xMidYMid meet"
					xmlns="http://www.w3.org/2000/svg"
					fill="currentColor"
					aria-hidden="true"
					width="20"
					height="20"
					viewBox="0 0 20 20"
					slot="icon"
					><!----><path
						d="M11.8 2.8L10.8 3.8 16.2 9.3 1 9.3 1 10.7 16.2 10.7 10.8 16.2 11.8 17.2 19 10z"
					/></svg
				><!----></dds-button-group-item
			>
			<!---->
			<dds-button-group-item
				aria-label=""
				href="https://example.com"
				data-autoid="dds--button-group-item"
				role="listitem"
				icon-layout=""
				kind="primary"
				><!---->Launch Engine<!----><svg
					focusable="false"
					preserveAspectRatio="xMidYMid meet"
					xmlns="http://www.w3.org/2000/svg"
					fill="currentColor"
					aria-hidden="true"
					width="20"
					height="20"
					viewBox="0 0 20 20"
					slot="icon"
					><!----><path
						d="M11.8 2.8L10.8 3.8 16.2 9.3 1 9.3 1 10.7 16.2 10.7 10.8 16.2 11.8 17.2 19 10z"
					/></svg
				><!----></dds-button-group-item
			>
			<!---->
		</dds-button-group>
		<dds-background-media
			mobile-position="bottom"
			opacity="100"
			data-autoid="dds--background-media"
			slot="image"
			gradient-direction="left-to-right"
		>
			<dds-video-player-container
				video-id="1_9h94wo6b"
				background-mode="true"
				playing-mode="inline"
				href="https://mediacenter.ibm.com/id/1_9h94wo6b"
				><!---->
				<dds-video-player
					hide-caption=""
					thumbnail-url="https://cdnsecakmi.kaltura.com/p/1773841/thumbnail/entry_id/1_9h94wo6b/width/3"
					video-id="1_9h94wo6b"
					playing-mode="inline"
					data-autoid="dds--video-player"
					content-state="thumbnail"
					background-mode="true"
					tabindex="0"
					duration="18"
					name="Test Video - 1:1"
					aria-label="Test Video - 1:1 (0:18)"
				>
					<div
						id="7njkbswq0ss"
						class="kWidgetIframeContainer bx--video-player__video"
						style="overflow: hidden;"
						data-video-id="1_9h94wo6b"
					>
						<iframe
							id="7njkbswq0ss_ifp"
							scrolling="no"
							name="7njkbswq0ss_ifp"
							class="mwEmbedKalturaIframe ibm-mediacenter-player ibm-mediacenter-player__1x1 ibm-mediacenter-player__video ibm-mediacenter-player__single-audio ibm-mediacenter-player__idl ibm-mediacenter-player__ibm-design-language"
							title="The Kaltura Dynamic Video Player"
							allow="autoplay *; fullscreen *; encrypted-media *"
							style="border: 0px none; max-width: 100%; max-height: 100%; width: 100%; height: 100%;"
							frameborder="0"
						/>
					</div></dds-video-player
				>
				<!----></dds-video-player-container
			>
		</dds-background-media>
	</dds-leadspace>
</Tile>

<Grid style="margin-top: -26px; width: 100%;">
	<Row style="margin-right: -2rem; margin-left: -3rem;">
		<Column aspectRatio="2x1">
			<AspectRatio ratio="16x9">
				<Tile style="height: 100% !important; padding: 0;">
					<YT
						title="Borderline teaser | Origin event"
						url="https://www.youtube.com/embed/uo8BfG6aw0g"
					/>
				</Tile>
			</AspectRatio>
		</Column>
		<Column aspectRatio="2x1">
			<FAQ />
		</Column>
	</Row>
</Grid>
