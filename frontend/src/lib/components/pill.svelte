<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import "$lib/css/pill.css";
  import { mdiHelp, mdiPlus } from "@mdi/js";
  import { shouldColorBeWhite } from "$lib/js/color.js";
  import { showPopup } from "$lib/js/popup.js";

  export let name = "Unknown";
  export let icon = mdiHelp;
  export let color = "#000000";
  export let type_icon = "mdi";
  export let shadow_color = null;
  export let show_tooltip = false;
  export let tooltip_data = [];

  const white = shouldColorBeWhite(color.slice(1));
  let style;
  let pill_arrow;
  let pill_tooltip;
  let main_pill;
  let innerWidth;

  if (shadow_color === null) {
    style = `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${color}60;`;
  } else {
    style = `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${shadow_color}60;`;
  }

  function showingTooltip(visible) {
    // Showing tooltip
    const isOutofBoundLeft=(main_pill.offsetLeft+(main_pill.offsetWidth/2)<(pill_tooltip.offsetWidth/2));
    const isOutofBoundRight=((pill_tooltip.offsetLeft+pill_tooltip.offsetWidth)>innerWidth);
    if (visible && tooltip_data.length > 0) {
      pill_tooltip.style.visibility = "visible";
      pill_arrow.style.visibility = "visible";
      if(isOutofBoundLeft)
      {
        pill_tooltip.style.left = "0";
      }
      if(isOutofBoundRight)
      {
        pill_tooltip.style.right = "0";
      }
      pill_tooltip.style.top = `${
        main_pill.offsetTop - pill_tooltip.offsetHeight - 16
      }px`;
    }
    // Hiding tooltip
    else {
      pill_tooltip.style.visibility = "hidden";
      pill_arrow.style.visibility = "hidden";
    }
  }
</script>

<svelte:window bind:innerWidth />

<div
  class={white ? "pill-container pill-white" : "pill-container pill-black"}
  {style}
  on:focus={() => true}
  on:mouseover={() => showingTooltip(true)}
  on:mouseleave={() => showingTooltip(false)}
  role="link"
  tabindex="0"
  bind:this={main_pill}
>
  {#if show_tooltip === true}
    <div class="pill-arrow" bind:this={pill_arrow} />
    <div class="pill-tooltip" bind:this={pill_tooltip}>
      {#each tooltip_data as td}
        <div>
          <span>{td.title}</span>
          <div class="pill-last">
            <button
              class="pill-view"
              on:click={() => showPopup(true, td.project_id)}
              ><SvgIcon size="20" path={mdiPlus} type="mdi" /></button
            >
          </div>
        </div>
      {/each}
    </div>
  {/if}
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
