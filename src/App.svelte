<script lang="ts">
  import "carbon-components-svelte/css/all.css";
  import { Button, Header, HeaderGlobalAction, HeaderUtilities, SkipToContent, Theme, Content } from "carbon-components-svelte";
  import Greet from './lib/Greet.svelte'

  import SettingsAdjust from "carbon-icons-svelte/lib/SettingsAdjust.svelte";
  import UserAvatarFilledAlt from "carbon-icons-svelte/lib/UserAvatarFilledAlt.svelte";
</script>

<Header company="IBM" platformName="Carbon Svelte">
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
  <HeaderUtilities>
    <HeaderGlobalAction aria-label="Settings" icon={SettingsAdjust} />
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