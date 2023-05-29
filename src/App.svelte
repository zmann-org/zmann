<script lang="ts">
  import "carbon-components-svelte/css/all.css";
  import { Button, Theme } from "carbon-components-svelte";
  import Greet from './lib/Greet.svelte'
</script>


<main class="container">
  <Theme
  theme="g90"
  tokens={{
    "interactive-01": "#00b4ffff",
    "hover-primary": "white",
    "active-primary": "#9f1853",
    "ui-background": "transparent",
  }}
/>
  <Button>Primary button</Button>
</main>