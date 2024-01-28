<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import "$lib/css/pill.css";
  import { mdiHelp } from "@mdi/js";
  import { shouldColorBeWhite } from "$lib/js/color.js";

  export let name = "Unknown";
  export let icon = mdiHelp;
  export let color = "#000000";
  export let type_icon = "mdi";
  export let shadow_color = null;

  const white = shouldColorBeWhite(color.slice(1));
  let style;
  if (shadow_color === null) {
    style = `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${color}60;`;
  } else {
    style = `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${shadow_color}60;`;
  }
</script>

<div
  class={white ? "pill-container pill-white" : "pill-container pill-black"}
  {style}
>
  {#if type_icon === "simpleicons"}
    <img
      height="20"
      width="20"
      src="https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/{icon}.svg"
      alt={name}
    />
  {:else}
    <SvgIcon type="mdi" path={icon} size="20" />
  {/if}
  <p>{name}</p>
</div>
