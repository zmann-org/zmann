<script>
	import { Tag } from 'carbon-components-svelte';
</script>

<div class="bx--grid">
	<div class="bx--row">
		<div class="bx--col-sm-4 bx--col-md-3 bx--col-lg-6 bx--col-xlg-4 bx--no-gutter">
			<dds-card
				border=""
				logo=""
				href="https://example.com"
				data-autoid="dds--card"
				color-scheme=""
				pictogram-placement="top"
			>
				<dds-image-logo
					slot="image"
					alt="Image alt text"
					default-src="https://www.ibm.com/standards/carbon/web-components/6fc681ddd902035ec0b80269cc27a907.png"
					data-autoid="dds-image-logo"
				/>

				<dds-card-eyebrow data-autoid="dds--card-eyebrow" slot="eyebrow"
					><!---->Microsoft<!----></dds-card-eyebrow
				>

				<p style="all:unset;">
					Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut
					aliquip ex ea commodo consequat.
				</p>

				<dds-tag-group data-autoid="dds--tag-group" role="list">
					<Tag type="green">Most popular</Tag>
					<Tag type="purple">Enterprise</Tag>
				</dds-tag-group>

				<dds-card-footer
					data-autoid="dds--card-footer"
					icon-placement="right"
					color-scheme=""
					slot="footer"
					parent-href="https://example.com"
					href="https://example.com"
				/>
			</dds-card>
		</div>
	</div>
</div>

<div class="bx--grid">
	<div class="bx--row">
		<div class="bx--col-lg-12 bx--no-gutter">
			<dds-video-cta-container>
				<dds-card-group
					cards-per-row="3"
					class="dds-ce-demo-devenv--cards-in-row-3"
					grid-mode="border"
					data-autoid="dds--card-group"
					style="--dds--card-group--cards-in-row: 3;"
				>
					<!---->
					<dds-card-group-item
						cta-type=""
						href=""
						color-scheme="light"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding-top: 1px; padding-left: 1px;"
					>
						<dds-card-eyebrow
							data-autoid="dds--card-eyebrow"
							slot="eyebrow"
							style="height: 18px;">Topic</dds-card-eyebrow
						>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							>Nunc convallis lobortis Nunc convallis lobortis Nunc convallis lobortis</dds-card-heading
						>
						<p style="height: 168px;">
							<dds-content-item-paragraph
								data-autoid="dds--content-item-paragraph"
								style="all:unset;"
								>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et
								ultricies est. Mauris iaculis eget dolor nec hendrerit. Phasellus at
								elit sollicitudin, sodales nulla quis, consequat libero.</dds-content-item-paragraph
							>
						</p>

						<dds-tag-group data-autoid="dds--tag-group" role="list">
							<bx-tag type="cool-gray"> Systems w/TPS </bx-tag>
							<bx-tag type="cool-gray"> Virtual </bx-tag>
						</dds-tag-group>

						<dds-text-cta
							slot="footer"
							cta-type="local"
							href="https://example.com"
							data-autoid="dds--text-cta"
							icon-placement="right"
						>
							Learn more
						</dds-text-cta>
					</dds-card-group-item>
					<!---->
					<dds-card-group-item
						cta-type=""
						href=""
						color-scheme="light"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding-top: 1px; padding-left: 0px;"
					>
						<dds-card-eyebrow
							data-autoid="dds--card-eyebrow"
							slot="eyebrow"
							style="height: 18px;">Topic</dds-card-eyebrow
						>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							><!---->Nunc convallis lobortis<!----></dds-card-heading
						>
						<p style="height: 168px;">
							<dds-content-item-paragraph
								data-autoid="dds--content-item-paragraph"
								style="all:unset;"
								>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et
								ultricies est.’</dds-content-item-paragraph
							>
						</p>

						<dds-tag-group data-autoid="dds--tag-group" role="list">
							<bx-tag type="cool-gray"> Systems w/TPS </bx-tag>
							<bx-tag type="cool-gray"> Virtual </bx-tag>
						</dds-tag-group>

						<dds-text-cta
							slot="footer"
							cta-type="local"
							href="https://example.com"
							data-autoid="dds--text-cta"
							icon-placement="right"
						>
							Learn more
						</dds-text-cta>
					</dds-card-group-item>
					<!---->
					<dds-card-group-item
						cta-type=""
						href=""
						color-scheme="light"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding-top: 1px; padding-left: 0px; padding-right: 1px;"
					>
						<dds-card-eyebrow
							data-autoid="dds--card-eyebrow"
							slot="eyebrow"
							style="height: 18px;">Topic</dds-card-eyebrow
						>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							><!---->Lorem ipsum dolor sit amet, consectetur.<!----></dds-card-heading
						>
						<p style="height: 168px;">
							<dds-content-item-paragraph
								data-autoid="dds--content-item-paragraph"
								style="all:unset;"
								>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et
								ultricies est.’</dds-content-item-paragraph
							>
						</p>

						<dds-tag-group data-autoid="dds--tag-group" role="list">
							<bx-tag type="cool-gray"> Systems w/TPS </bx-tag>
							<bx-tag type="cool-gray"> Virtual </bx-tag>
						</dds-tag-group>

						<dds-text-cta
							slot="footer"
							cta-type="local"
							href="https://example.com"
							data-autoid="dds--text-cta"
							icon-placement="right"
						>
							Learn more
						</dds-text-cta>
					</dds-card-group-item>
					<!---->
					<dds-card-group-item
						cta-type=""
						href=""
						color-scheme="light"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding-top: 0px; padding-bottom: 1px; padding-left: 1px;"
					>
						<dds-card-eyebrow
							data-autoid="dds--card-eyebrow"
							slot="eyebrow"
							style="height: 18px;">Topic</dds-card-eyebrow
						>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							><!---->Te sint disputando pri, at his aliquip corrumpit<!----></dds-card-heading
						>
						<p style="height: 168px;">
							<dds-content-item-paragraph
								data-autoid="dds--content-item-paragraph"
								style="all:unset;"
								>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et
								ultricies est.’</dds-content-item-paragraph
							>
						</p>

						<dds-tag-group data-autoid="dds--tag-group" role="list">
							<bx-tag type="cool-gray"> Systems w/TPS </bx-tag>
							<bx-tag type="cool-gray"> Virtual </bx-tag>
						</dds-tag-group>

						<dds-text-cta
							slot="footer"
							cta-type="local"
							href="https://example.com"
							data-autoid="dds--text-cta"
							icon-placement="right"
						>
							Learn more
						</dds-text-cta>
					</dds-card-group-item>
					<!---->
					<dds-card-group-item
						cta-type=""
						href=""
						color-scheme="light"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding-top: 0px; padding-bottom: 1px; padding-left: 0px;"
					>
						<dds-card-eyebrow
							data-autoid="dds--card-eyebrow"
							slot="eyebrow"
							style="height: 18px;">Topic</dds-card-eyebrow
						>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							><!---->Disputando lorem covallis<!----></dds-card-heading
						>
						<p style="height: 168px;">
							<dds-content-item-paragraph
								data-autoid="dds--content-item-paragraph"
								style="all:unset;"
								>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean et
								ultricies est.’</dds-content-item-paragraph
							>
						</p>

						<dds-tag-group data-autoid="dds--tag-group" role="list">
							<bx-tag type="cool-gray"> Systems w/TPS </bx-tag>
							<bx-tag type="cool-gray"> Virtual </bx-tag>
						</dds-tag-group>

						<dds-text-cta
							slot="footer"
							cta-type="local"
							href="https://example.com"
							data-autoid="dds--text-cta"
							icon-placement="right"
						>
							Learn more
						</dds-text-cta>
					</dds-card-group-item>
					<!---->
					<dds-card-group-item
						cta-type="local"
						href="https://example.com"
						color-scheme="inverse"
						data-autoid="dds--card-group-item"
						border=""
						pictogram-placement="top"
						style="padding: 0px 1px 1px 0px;"
					>
						<dds-card-heading
							role="heading"
							aria-level="3"
							data-autoid="dds--card-heading"
							slot="heading"
							style="height: 84px; margin-bottom: 88px;"
							>Top level card link</dds-card-heading
						>
						<dds-card-cta-footer
							slot="footer"
							color-scheme="inverse"
							data-autoid="dds--card-cta-footer"
							icon-placement="right"
							cta-type="local"
							parent-href="https://example.com"
							href="https://example.com"
							style="height: 23px;"
						/>
					</dds-card-group-item>
					<!---->
				</dds-card-group>
			</dds-video-cta-container>
		</div>
	</div>
</div>
