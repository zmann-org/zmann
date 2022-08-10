<script>
	import { Accordion, AccordionItem, Link } from 'carbon-components-svelte';
	import Restart from 'carbon-icons-svelte/lib/Restart.svelte';
</script>

<Accordion>
	<AccordionItem open>
		<svelte:fragment slot="title">
			<h5>What is Vacaro?</h5>
			<div>Engine / General Knowledge</div>
		</svelte:fragment>
		<p>
			Vacaro is a game and video engine, powered to achieve a realistic and efficient game
			experience. <strong
				>Vacaro is still in development and can have bugs or performance issues.</strong
			>
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>How do I get started?</h5>
			<div>Engine / General Knowledge</div>
		</svelte:fragment>
		<p>
			Analyze text to extract meta-data from content such as concepts, entities, emotion,
			relations, sentiment and more.
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>What engine version should I install?</h5>
			<div>Dev-ops / Launcher Question</div>
		</svelte:fragment>
		<p>
			Translate text, documents, and websites from one language to another. Create industry or
			region-specific translations via the service's customization capability.
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>What engine version should I install?</h5>
			<div>Dev-ops / Launcher Question</div>
		</svelte:fragment>
		<p>
			Translate text, documents, and websites from one language to another. Create industry or
			region-specific translations via the service's customization capability.
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>What engine version should I install?</h5>
			<div>Dev-ops / Launcher Question</div>
		</svelte:fragment>
		<p>
			Translate text, documents, and websites from one language to another. Create industry or
			region-specific translations via the service's customization capability.
		</p>
	</AccordionItem>
	<div style="text-align: center; margin-bottom: 50px; margin-top: 10px;">
	<Link icon={Restart}>Load more Questions</Link>
</div>
</Accordion>
