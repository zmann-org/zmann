<script>
	import Footer from '../components/navigation/Footer.svelte';

	import Cards from '../components/engine/Cards.svelte';
	import Search from '../components/engine/Search.svelte';
</script>

<Search />
<Cards />

<dds-video-cta-container class="bx--grid" style="margin-top: 50px;">
	<div class="bx--row">
		<div class="bx--col-sm-4 bx--col-lg-6">
			<dds-link-list type="end" data-autoid="dds--link-list">
				<dds-link-list-heading
					role="heading"
					aria-level="4"
					data-autoid="dds--link-list-heading"
					slot="heading">Tutorials</dds-link-list-heading
				>
				<dds-link-list-item
					href="https://example.com"
					role="listitem"
					data-autoid="dds--link-list-item"
					icon-placement="right"
					type="end"
				>
					Learn more about Kubernetes <svg
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
					>
				</dds-link-list-item>
				<dds-link-list-item
					href="https://example.com"
					role="listitem"
					data-autoid="dds--link-list-item"
					icon-placement="right"
					type="end"
				>
					Containerization A Complete Guide <svg
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
					>
				</dds-link-list-item>
				<dds-link-list-item
					href="https://example.com"
					role="listitem"
					data-autoid="dds--link-list-item"
					icon-placement="right"
					type="end"
				>
					Microservices and containers <svg
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
					>
				</dds-link-list-item>
			</dds-link-list>
		</div>
	</div>
</dds-video-cta-container>


<!-- <Footer /> -->
