<script>
  /** Set to `true` to use the active variant */
  export let isActive = false;

  /**
   * Specify the icon to render
   * @type {typeof import("svelte").SvelteComponent}
   */
  export let icon = undefined;

  /** Obtain a reference to the HTML button element */
  export let ref = null;
</script>

<div
  class:bx--header__action={true}
  style="display: flex;
justify-content: center;
align-items: center;
}"
>
  <button
    type="button"
    bind:this={ref}
    {...$$restProps}
    class:bx--btn--primary={true}
    class:bx--header__action={true}
    class:bx--header__action--active={isActive}
    {...$$restProps}
    on:click
    style="background-color: var(--cds-interactive-01, #0f62fe) !important; width: 36px; height: 36px;">
    <slot>
      <svelte:component this={icon} size={24} style="vertical-align: middle;"/>
    </slot>
  </button>
</div>

<style>
  .bx--header__action:focus {
    border-color: transparent !important;
    outline: none;
  }
  .bx--header__action:hover {
    background-color: transparent !important;
  }
</style>
