<script lang="ts">
  import "carbon-components-svelte/css/all.css";
  import {
    Button,
    SkipToContent,
    Theme,
    Content,
  } from "carbon-components-svelte";
  import { Header, HeaderUtilities, HeaderTitlebar } from "./shell";
</script>

<Header platformName="Distraze">
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
  <HeaderUtilities>
    <HeaderTitlebar />
  </HeaderUtilities>
</Header>

<Content>
  <Theme
    theme="g90"
    tokens={{
      "interactive-01": "#00b4ffff",
      "hover-primary": "white",
      "ui-background": "transparent",
    }}
  />
  <Button>Primary button</Button>
</Content>
