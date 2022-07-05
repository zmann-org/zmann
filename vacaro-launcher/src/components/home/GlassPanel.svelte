<script>
	import { ProgressIndicator, ProgressStep, Grid, Row, Column } from 'carbon-components-svelte';
</script>

<div>
	<ProgressIndicator preventChangeOnClick currentIndex={2}>
		<ProgressStep
			complete
			label="Step 1"
			description="The progress indicator will listen for clicks on the steps"
		/>
		<ProgressStep
			complete
			label="Step 2"
			description="The progress indicator will listen for clicks on the steps"
		/>
		<ProgressStep
			complete
			label="Step 3"
			description="The progress indicator will listen for clicks on the steps"
		/>
		<ProgressStep
			label="Step 4"
			description="The progress indicator will listen for clicks on the steps"
		/>
	</ProgressIndicator>
</div>
