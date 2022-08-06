<script>
	import {
		Grid,
		Row,
		Column,
		Tile,
		AspectRatio,
		ButtonSet,
		Dropdown,
		Button,
		InlineNotification,
NotificationActionButton
	} from 'carbon-components-svelte';
	import Launch from 'carbon-icons-svelte/lib/Launch.svelte';

	import Banner from '../components/home/Banner.svelte';
	import Carousel from '../components/home/Carousel.svelte';
	import FAQ from '../components/home/FAQ.svelte';
	import YT from '../components/home/YT.svelte';
	import FeaturesSection from '../components/home/FeaturesSection.svelte';
	import Sponsors from '../components/home/Sponsors.svelte';
	import GroupBanner from '../components/home/GroupBanner.svelte';

	import { relaunch } from '@tauri-apps/api/process';

	import { breakpointObserver, breakpoints } from 'carbon-components-svelte';

	const size = breakpointObserver();
	const smaller = size.smallerThan('max');
</script>

<!-- <Banner /> -->
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
				><!---->Manage Projects<!----><Launch
					style="margin-left: 6px; margin-right: -49px;"
				/><!----></dds-button-group-item
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
			alt="Image alt text"
			default-src="/video/splash.png"
			data-autoid="dds--background-media"
			slot="image"
			gradient-direction="left-to-right"
		>
			<dds-image-item
				media="(min-width: 1800px)"
				srcset="/video/splash1.png"
				data-autoid="dds-image-item"
			/>
			<dds-image-item
				media="(min-width: 1600px)"
				srcset="/video/splash2.png"
				data-autoid="dds-image-item"
			/>
			<dds-image-item
				media="(min-width: 1400px)"
				srcset="/video/splash3.png"
				data-autoid="dds-image-item"
			/>
			<dds-image-item
				media="(min-width: 1200px)"
				srcset="/video/splash4.png"
				data-autoid="dds-image-item"
			/>
			<dds-image-item
				media="(min-width: 1060px)"
				srcset="/video/splash.png"
				data-autoid="dds-image-item"
			/>
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
			{#if $smaller}
				<InlineNotification
					hideCloseButton
					kind="info"
					title="Vacaro is in early development,"
					subtitle="the codebase and API can change rapidly."
				>
					<svelte:fragment slot="actions">
						<NotificationActionButton>Visit the Roadmap</NotificationActionButton>
					</svelte:fragment>
				</InlineNotification>
			{/if}
		</Column>
		<Column aspectRatio="2x1">
			<FAQ />
		</Column>
	</Row>
</Grid>
<!-- <Carousel /> -->

<GroupBanner />
<FeaturesSection />

<!-- <Sponsors /> -->
