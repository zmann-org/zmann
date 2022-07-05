<script>
	import { Accordion, AccordionItem } from 'carbon-components-svelte';
</script>

<Accordion>
	<AccordionItem open>
		<svelte:fragment slot="title">
			<h5>Natural Language Classifier</h5>
			<div>AI / Machine Learning</div>
		</svelte:fragment>
		<p>
			Natural Language Classifier uses advanced natural language processing and machine learning
			techniques to create custom classification models. Users train their data and the service
			predicts the appropriate category for the inputted text.
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>Natural Language Understanding</h5>
			<div>AI / Machine Learning</div>
		</svelte:fragment>
		<p>
			Analyze text to extract meta-data from content such as concepts, entities, emotion, relations,
			sentiment and more.
		</p>
	</AccordionItem>
	<AccordionItem>
		<svelte:fragment slot="title">
			<h5>Language Translator</h5>
			<div>AI / Machine Learning</div>
		</svelte:fragment>
		<p>
			Translate text, documents, and websites from one language to another. Create industry or
			region-specific translations via the service's customization capability.
		</p>
	</AccordionItem>
</Accordion>
