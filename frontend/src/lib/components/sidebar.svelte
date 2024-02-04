<script>
  import { onMount } from "svelte";
  import SvgIcon from "@jamescoyle/svelte-icon";
  import {
    mdiAccount,
    mdiCogs,
    mdiEmailOutline,
    mdiPhone,
    mdiStar,
    mdiClose,
  } from "@mdi/js";

  import SidebarComponent from "$lib/components/sidebar-component.svelte";
  import { formatDate } from "$lib/js/date.js";
  import { showSidebar } from "$lib/js/topbar.js";
  import "$lib/css/sidebar.css";

  export let info;
  export let text;
  export let footer = null;
  export let containerCv = null;
  export let sidebarContainer;
  let sidebar;
  let birth_year;
  if (info.birth_year != null) {
    birth_year = formatDate(info.birth_year);
  }
  $: scrollY = 0;
  $: innerHeight = 0;

  function sidebarScrollingHandler() {
    // Disabled some functionnalities if no footer nor containerCv is provided
    if (footer === null || containerCv === null) {
      return;
    }

    let isBottom = scrollY + innerHeight >= footer.offsetTop;
    let isMoved = scrollY + innerHeight >= sidebar.offsetHeight;
    let littleScreen = innerHeight < sidebar.offsetHeight;
    // Only having the sticky sidebar if the size of the screen is too 'little'

    // Testing if sidebar is outside of scrolling scope
    if (isMoved && !isBottom) {
      sidebar.style.position = "fixed";
      sidebar.style.top = littleScreen
        ? `${innerHeight - sidebar.offsetHeight}px`
        : "0px";
    }
    // Checking if at the bottom, calculating the diff. between the cv and the sidebar heights
    else if (isBottom && littleScreen) {
      sidebar.style.position = "absolute";
      sidebar.style.top = `${
        containerCv.offsetHeight - sidebar.offsetHeight
      }px`;
    }
    // Only putting absolute if on little screen
    else if (littleScreen) {
      sidebar.style.position = "absolute";
      sidebar.style.top = "";
    }
  }

  onMount(async () => {
    sidebarScrollingHandler();
  });
</script>

<svelte:window
  bind:scrollY
  bind:innerHeight
  on:scroll={() => {
    sidebarScrollingHandler();
  }}
/>

<!-- if no footer nor containerCv is provided we are in mobile mode -->
{#if footer === null || containerCv === null}
  <div id="sidebar-container" bind:this={sidebarContainer}>
    <button on:click={() => showSidebar()}>
      <SvgIcon size="23" path={mdiClose} type="mdi" />
    </button>
    <div class="sidebar">
      <div class="sidebar-profilepic-container">
        <img
          class="sidebar-profilepic"
          src={info.profile_pic}
          alt={info.full_name}
        />
      </div>
      {#if info.birth_year != null}
        <SidebarComponent icon={mdiAccount} description={birth_year} />
      {/if}
      {#if info.email != null}
        <SidebarComponent icon={mdiEmailOutline} description={info.email} />
      {/if}
      {#if info.phone_number != null}
        <SidebarComponent icon={mdiPhone} description={info.phone_number} />
      {/if}
      {#if info.interests != null}
        <SidebarComponent
          icon={mdiStar}
          title={text.interests}
          description={info.interests}
        />
      {/if}
      {#if info.interests != null}
        <SidebarComponent
          icon={mdiCogs}
          title={text.softskills}
          description={info.softskills}
        />
      {/if}
    </div>
  </div>
{:else}
  <div class="sidebar" bind:this={sidebar}>
    <div class="sidebar-profilepic-container">
      <img
        class="sidebar-profilepic"
        src={info.profile_pic}
        alt={info.full_name}
      />
    </div>
    {#if info.birth_year != null}
      <SidebarComponent icon={mdiAccount} description={birth_year} />
    {/if}
    {#if info.email != null}
      <SidebarComponent icon={mdiEmailOutline} description={info.email} />
    {/if}
    {#if info.phone_number != null}
      <SidebarComponent icon={mdiPhone} description={info.phone_number} />
    {/if}
    {#if info.interests != null}
      <SidebarComponent
        icon={mdiStar}
        title="Interests"
        description={info.interests}
      />
    {/if}
    {#if info.interests != null}
      <SidebarComponent
        icon={mdiCogs}
        title="Soft-Skills"
        description={info.softskills}
      />
    {/if}
  </div>
  <div class="fake-sidebar" />
{/if}
