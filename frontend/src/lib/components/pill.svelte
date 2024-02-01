<script>
  import { onMount } from "svelte";
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
  let style =
    shadow_color === null
      ? `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${color}60;`
      : `background-color: ${color};
    box-shadow: 0px 8px 18px -1px ${shadow_color}60;`;

  // pill elements from DOM
  let pill_arrowup;
  let pill_arrowdown;
  let pill_tooltip;
  let main_pill;

  // constants and variables
  let innerWidth;
  let scrollY;
  let offsetUp;
  let isOutofBoundLeft;
  let isOutofBoundRight;

  function showingTooltip(visible) {
    calculateOutOfBounds();
    // Showing tooltip
    if (visible && tooltip_data.length > 0) {
      // forcing left or right depending on the out of bound situation
      if (isOutofBoundLeft) {
        pill_tooltip.style.left = "0";
      }
      if (isOutofBoundRight) {
        pill_tooltip.style.right = "0";
      }
      // Setting the top size depending on the situation
      if (scrollY > offsetUp) {
        // 51 represents the size of the pill
        pill_tooltip.style.top = `${main_pill.offsetTop + 51 + 16}px`;
        pill_arrowdown.style.visibility = "visible";
      } else {
        pill_tooltip.style.top = `${offsetUp}px`;
        pill_arrowup.style.visibility = "visible";
      }
      pill_tooltip.style.visibility = "visible";
    }
    // Hiding tooltip
    else {
      pill_tooltip.style.visibility = "hidden";
      pill_arrowup.style.visibility = "hidden";
      pill_arrowdown.style.visibility = "hidden";
    }
  }

  function calculateOutOfBounds() {
    if (show_tooltip && tooltip_data.length > 0) {
      // outofbound for left
      isOutofBoundLeft =
        main_pill.offsetLeft + main_pill.offsetWidth / 2 <
        pill_tooltip.offsetWidth / 2;
      // outofbound for right
      isOutofBoundRight =
        pill_tooltip.offsetLeft + pill_tooltip.offsetWidth > innerWidth;
    }
  }

  function calculateOffsetUp() {
    // 16 = arrow size + something
    if (show_tooltip && tooltip_data.length > 0) {
      pill_tooltip.style.top = "";
      pill_tooltip.style.left = "";
      pill_tooltip.style.right = "";
      offsetUp = main_pill.offsetTop - pill_tooltip.clientHeight - 16;
    }
  }

  onMount(async () => {
    calculateOffsetUp();
  });
</script>

<svelte:window
  bind:innerWidth
  bind:scrollY
  on:resize={() => {
    console.log("tamere");
    calculateOffsetUp();
    calculateOutOfBounds();
  }}
/>

{#if show_tooltip && tooltip_data.length > 0}
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
      <div class="pill-arrow pill-arrow-up" bind:this={pill_arrowup} />
      <div class="pill-arrow pill-arrow-down" bind:this={pill_arrowdown} />
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
{:else}
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
{/if}
