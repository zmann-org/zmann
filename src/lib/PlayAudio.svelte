<script>
  import { Button } from "carbon-components-svelte";
  import { invoke } from "@tauri-apps/api/tauri"

  function invokeAudio(){
    invoke("play_audio")
  }
</script>

<Button on:click={invokeAudio}>Play Rodio Audio</Button>
