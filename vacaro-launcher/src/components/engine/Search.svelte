<dds-leadspace-with-search
	adjacent-theme=""
	style="margin-top: -32px !important;"
	data-autoid="dds--leadspace-with-search"
	has-image=""
>
	<dds-background-media
		gradient-direction="left-to-right"
		mobile-position="bottom"
		slot="image"
		data-autoid="dds--background-media"
		opacity="100"
		default-src="/video/splash1.png"
	>
		<dds-image-item
			media="(min-width: 1584px)"
			srcset="https://fpoimg.com/1584x560?&bg_color=ee5396&text_color=161616"
		/>
		<dds-image-item media="(min-width: 1312px)" srcset="/video/splash1.png" />
		<dds-image-item media="(min-width: 672px)" srcset="/video/splash1.png" />
		<dds-image-item media="(min-width: 320px)" srcset="/video/splash1.png" />
		<dds-image-item media="(min-width: 0px)" srcset="/video/splash1.png" />
	</dds-background-media>
	<dds-leadspace-with-search-heading
		role="heading"
		aria-level="1"
		data-autoid="dds--leadspace-with-search__heading"
		slot="heading"><!---->Find a product<!----></dds-leadspace-with-search-heading
	>
	<dds-leadspace-with-search-content
		data-autoid="dds--leadspace-with-search__content"
		slot="content"
	>
		<dds-leadspace-with-search-content-heading
			role="heading"
			aria-level="2"
			data-autoid="dds--leadspace-with-search-content-heading"
			slot="heading"
			><!---->
			Innovate like a startup, scale for the enterprise
			<!----></dds-leadspace-with-search-content-heading
		>
		<dds-leadspace-with-search-content-copy
			style="display: none"
			data-autoid="dds--leadspace-with-search-content-copy"
			slot="copy"
			><!---->
		</dds-leadspace-with-search-content-copy>
	</dds-leadspace-with-search-content>
	<dds-search-with-typeahead
		slot="search"
		leadspace-search=""
		data-autoid="dds--search-with-typeahead"
		color-scheme=""
		type=""
		value=""
		should-remain-open=""
		active=""
	/>
	<dds-hr
		slot="hr"
		style=""
		data-autoid="dds--hr"
		type="solid"
		size="fluid"
		contrast="medium-contrast"
		weight="thin"
	/>
</dds-leadspace-with-search>
