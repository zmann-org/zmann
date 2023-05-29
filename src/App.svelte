<script lang="ts">
  import "carbon-components-svelte/css/all.css";
  import { Button, HeaderGlobalAction, SkipToContent, Theme, Content } from "carbon-components-svelte";
  import { Header, HeaderUtilities, HeaderTitleButton } from './shell'
  import Greet from './lib/Greet.svelte'

  import SettingsAdjust from "carbon-icons-svelte/lib/SettingsAdjust.svelte";
  import UserAvatarFilledAlt from "carbon-icons-svelte/lib/UserAvatarFilledAlt.svelte";
  import Close from "carbon-icons-svelte/lib/Close.svelte";
</script>

<Header platformName="Distraze">
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
  <HeaderUtilities>
    <HeaderGlobalAction aria-label="Settings" icon={SettingsAdjust} />
    <HeaderTitleButton aria-label="Settings" icon={Close} />
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