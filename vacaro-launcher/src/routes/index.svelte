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

<Tile
	style="height: 400px; background-position: center; background-size: 200rem; background-image: url(https://media.discordapp.net/attachments/281535090134351874/985849155912101939/xxcww.jpg);"
>
	<Grid narrow>
		<Row>
			<Column>
				<AspectRatio
					ratio="16x9"
					style="background-color: #ffffff17; max-height: 368px; backdrop-filter: blur(50px);"
				>
					<GlassPanel/>
				</AspectRatio>
			</Column>
			<Column sm={{ offset: 10 }}>
				<ButtonSet stacked>
					<Button icon={Launch}>Launch Engine</Button>
					<Dropdown
						hideLabel
						light
						type="default"
						style="width: 100%; max-width: 12.25rem;"
						titleText="Engine Version"
						selectedId="0"
						items={[
							{ id: '0', text: '0.0.1' },
							{ id: '1', text: '0.0.2 Beta' },
							{ id: '2', text: '0.0.3 Alpha' }
						]}
					/>
				</ButtonSet>
			</Column>
		</Row>
	</Grid>
</Tile>

<Grid style="margin-top: 12px; width: 100%;">
	<Row>
		<Column aspectRatio="2x1">
			<AspectRatio ratio="16x9">
				<Tile style="height: 100% !important;">
					<YT title="Music" url="https://www.youtube.com/embed/_8_X-M68c6Q" />
				</Tile>
			</AspectRatio>
		</Column>
		<Column aspectRatio="2x1">
			<FAQ />
		</Column>
	</Row>
</Grid>
