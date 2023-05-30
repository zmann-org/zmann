<script lang="ts">
  import "carbon-components-svelte/css/all.css";
  import {
    Button,
    SkipToContent,
    Theme,
    Content,
    HeaderNav,
    HeaderNavItem,
  } from "carbon-components-svelte";
  import { Header, HeaderUtilities, HeaderTitlebar } from "./shell";
  import { trace, info, error, attachConsole } from "tauri-plugin-log-api";
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri'
  import Greet from "./lib/Greet.svelte";

  onMount(() => {
    info("onMount => http://localhost:1420/");
  });
</script>

<Header platformName="Distraze">
  <svelte:fragment slot="skip-to-content">
    <SkipToContent />
  </svelte:fragment>
  <HeaderNav>
    <HeaderNavItem text="Home" isSelected />
    <HeaderNavItem text="Home" />
    <HeaderNavItem text="Home" />
  </HeaderNav>
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
<Greet/>
</Content>
