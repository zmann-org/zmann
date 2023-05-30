<script>
  import { HeaderTitleButton, HeaderTitleButtonSmall, HeaderTitleButtonSmallRounded } from "../shell";

  import Minimize from "carbon-icons-svelte/lib/Minimize.svelte";
  import Subtract from "carbon-icons-svelte/lib/Subtract.svelte";
  import Launch from "carbon-icons-svelte/lib/Launch.svelte";
  import Close from "carbon-icons-svelte/lib/Close.svelte";
  import Renew from "carbon-icons-svelte/lib/Renew.svelte";
  import { appWindow } from "@tauri-apps/api/window";

  let isMaximized = false;

  function toggleApp() {
    appWindow.toggleMaximize();
    isMaximized = !isMaximized;
  }

  appWindow.onResized(() => {
    appWindow.isMaximized().then((e) => {
      if (e == true) isMaximized = true;
      else isMaximized = false;
    });
  });
</script>

<HeaderTitleButton
  aria-label="Settings"
  icon={Renew}
  on:click={() => appWindow.close()}
/>

<HeaderTitleButtonSmall
  aria-label="Settings"
  icon={Subtract}
  on:click={() => appWindow.minimize()}
/>
<HeaderTitleButton
  aria-label="Settings"
  icon={isMaximized ? Minimize : Launch}
  on:click={() => toggleApp()}
/>

<HeaderTitleButtonSmallRounded
  aria-label="Settings"
  icon={Close}
  on:click={() => appWindow.close()}
/>
