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

<div class:bx--header__action={true}>
  <button
    type="button"
    bind:this={ref}
    class:bx--header__action={true}
    class:bx--header__action--active={isActive}
    {...$$restProps}
    on:click
    style="background-color: var(--cds-interactive-01, #0f62fe) !important;     width: 35px;
  height: 35px;"
  >
    <slot>
      <svelte:component this={icon} size={20} />
    </slot>
  </button>
</div>

<style>
  .bx--header__action:focus {
    border-color: transparent !important;
    outline: none;
  }
</style>
