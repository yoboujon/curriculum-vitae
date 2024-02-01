<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import { processData } from "$lib/js/processdata.js";
  import { showSidebar } from "$lib/js/topbar.js";
  import "$lib/css/base.css";
  import "$lib/css/cv.css";

  // Main
  import Sidebar from "$lib/components/sidebar.svelte";
  import Section from "$lib/components/section.svelte";
  import SubSection from "$lib/components/subsection.svelte";
  import Education from "$lib/components/education.svelte";
  import Experience from "$lib/components/experience.svelte";
  import Projects from "$lib/components/projects.svelte";
  import SlideShow from "$lib/components/slideshow.svelte";
  import Pill from "$lib/components/pill.svelte";
  import FlagComponent from "$lib/components/flag-component.svelte";
  import ProjectsPopup from "$lib/components/projects-popup.svelte";
  import {
    mdiSchool,
    mdiBriefcase,
    mdiWrench,
    mdiPencil,
    mdiAccount,
    mdiXml,
    mdiApplication,
    mdiEarth,
    mdiHeart,
  } from "@mdi/js";
  import { onMount } from "svelte";

  export let data;
  const cv = data.status == 0 ? processData(data) : undefined;

  // Sidebar
  let containerCv;
  let footer;

  // Mobile top bar
  function mobileTopBar() {
    // 53 px or half the topbar size
    if (scrollY > 53) {
      topbar.style.height = "53px";
      topbar.style.backgroundColor = "#F8F1F1AE";
      topbar.style.boxShadow = "0px 8px 18px -1px #1d4a6560";
      buttonTopbar.style.display = "flex";
    } else {
      topbar.style.height = "";
      topbar.style.backgroundColor = "#F8F1F100";
      topbar.style.boxShadow = "";
      buttonTopbar.style.display = "none";
    }
  }

  // Mobile check
  $: innerWidth = 0;
  $: scrollY = 0;
  let topbar;
  let sidebarContainer;
  let buttonTopbar;

  onMount(async () => {
    mobileTopBar();
  });
</script>

<svelte:window
  bind:scrollY
  bind:innerWidth
  on:scroll={() => {
    if (innerWidth < 1200) {
      mobileTopBar();
    }
  }}
/>

{#if data.status == 0}
  <ProjectsPopup tags={cv.tags} />
  <!-- TOPBAR DIV (POPUP: mobile) -->
  {#if innerWidth < 1200}
    <Sidebar info={cv.info} bind:sidebarContainer />
  {/if}
  <div class="container-cv" bind:this={containerCv}>
    <!-- SIDEBAR DIV (LEFT: desktop) -->
    {#if innerWidth >= 1200}
      <Sidebar info={cv.info} {footer} {containerCv} />
    {/if}
    <!-- MOBILE TOP BAR -->
    {#if innerWidth < 1000}
      <div id="topbar" bind:this={topbar}>
        <button on:click={() => showSidebar(true)} bind:this={buttonTopbar}>
          <SvgIcon size="23" path={mdiAccount} type="mdi" />
        </button>
        <h1 class={scrollY <= 53 ? "topbar-name" : "topbar-name-little"}>
          {cv.info.full_name}
        </h1>
      </div>
      <div id="fake-topbar" />
    {/if}
    <!-- MAIN DIV (RIGHT: desktop/CENTER: mobile) -->
    <div class="main">
      {#if innerWidth >= 1000}
        <h1 class="name">{cv.info.full_name}</h1>
      {/if}
      <h2 class="name">Apprentice Engineer Automatic/Electronic</h2>
      <Section icon={mdiSchool} title="Education" />
      <SlideShow
        data={cv.education}
        type={Education}
        timeline="true"
        reverse="true"
      />
      <Section icon={mdiBriefcase} title="Experience" />
      <SlideShow
        data={cv.experiences}
        type={Experience}
        timeline="true"
        reverse="true"
      />
      <Section icon={mdiWrench} title="Projects" />
      <SlideShow data={cv.skills.project} type={Projects} />
      <Section icon={mdiPencil} title="Skills" />
      <SubSection icon={mdiXml} title="Programming Languages" />
      <div class="subsection">
        {#each cv.skills.programming_languages as pilldata, index (index)}
          <Pill
            name={pilldata.lang}
            type_icon={pilldata.type_icon}
            icon={pilldata.icon}
            color={pilldata.color}
            show_tooltip={true}
            tooltip_data={cv.project_programming[index]}
          />
        {/each}
      </div>
      <SubSection icon={mdiApplication} title="Software" />
      <div class="subsection">
        {#each cv.skills.softwares as pilldata, index (index)}
          <Pill
            name={pilldata.software}
            type_icon={pilldata.type_icon}
            icon={pilldata.icon}
            color={pilldata.color}
            show_tooltip={true}
            tooltip_data={cv.project_software[index]}
          />
        {/each}
      </div>
      <SubSection icon={mdiEarth} title="Languages" />
      <div class="subsection flag-container end">
        {#each cv.skills.languages as langdata}
          <FlagComponent
            lang={langdata.lang}
            level={langdata.level}
            icon={langdata.icon_alpha}
          />
        {/each}
      </div>
    </div>
  </div>
  <div class="footer" bind:this={footer}>
    <p>
      Made with <SvgIcon size="20" path={mdiHeart} type="mdi" /> using Svelte
    </p>
    <p>All rights reserved, Yohan Boujon • {new Date().getFullYear()}</p>
  </div>
{:else}
  <h1 class="h1 text-center">Oops, could not load database :/</h1>
{/if}
