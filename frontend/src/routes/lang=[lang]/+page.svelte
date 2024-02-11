<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import { showSidebar } from "$lib/js/topbar.js";
  import "$lib/css/base.css";
  import "$lib/css/cv.css";
  import "$lib/css/pill.css";
  import "/node_modules/flag-icons/css/flag-icons.min.css";

  // Main
  import Section from "$lib/components/section.svelte";
  import SubSection from "$lib/components/subsection.svelte";
  import Education from "$lib/components/education.svelte";
  import Experience from "$lib/components/experience.svelte";
  import Projects from "$lib/components/projects.svelte";
  import SlideShow from "$lib/components/slideshow.svelte";
  import FlagComponent from "$lib/components/flag-component.svelte";
  import ProjectsPopup from "$lib/components/projects-popup.svelte";
  import Pill from "$lib/components/pill.svelte";
  import {
    mdiSchool,
    mdiBriefcase,
    mdiWrench,
    mdiPencil,
    mdiAccount,
    mdiEarth,
    mdiHeart,
    mdiArrowDown,
    mdiArrowUp,
  } from "@mdi/js";
  import { onMount } from "svelte";

  export let data;
  // Database
  const cv = data.status == 0 ? data : undefined;

  // Language specifications
  const text = data.text;
  let flag;
  let otherlang;
  if (data.status == 0) {
    for (const lang of cv.languages) {
      if (lang.url_name == data.lang) flag = lang.icon_alpha;
      else otherlang = lang.url_name;
    }
  }

  // Sidebar
  let containerCv;
  let footer;
  $: sidebarLoaded = false;
  let Sidebar;

  // Mobile top bar
  function mobileTopBar() {
    // 53 px or half the topbar size
    if (scrollY > 53) {
      topbar.style.height = "53px";
      topbar.style.backgroundColor = "#F8F1F1AE";
      topbar.style.boxShadow = "0px 8px 18px -1px #1d4a6560";
      buttonTopbar.style.backgroundImage = "";
    } else {
      topbar.style.height = "";
      topbar.style.backgroundColor = "#F8F1F100";
      topbar.style.boxShadow = "";
      buttonTopbar.style.backgroundImage = `url('${cv.info.profile_pic}')`;
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
    Sidebar = (await import("/src/lib/components/sidebar.svelte")).default;
    sidebarLoaded = true;
  });

  // More pill handler
  let pillShowMore = [];
  let pillInstances = new Array();
  for (let i = 0; i < cv.skills.length; i++) {
    pillInstances[i] = new Array();
  }

  async function moreHandler(id) {
    if (pillShowMore[id] === true) {
      // show Less
      pillShowMore[id] = false;
    } else {
      // Show More
      pillShowMore[id] = true;
    }
    await new Promise((resolve) => setTimeout(resolve, 30));
    for (const pills of pillInstances) {
      for (const p of pills) {
        p.updatePill();
      }
    }
  }

  function needMore(skill) {
    for (const s of skill) {
      if (!s.is_shown) return true;
    }
    return false;
  }
</script>

<svelte:window
  bind:scrollY
  bind:innerWidth
  on:scroll={() => {
    if (innerWidth < 1200) {
      mobileTopBar();
    }
  }}
  on:resize={() => {
    if (innerWidth < 1200) {
      mobileTopBar();
    }
  }}
/>

{#if data.status == 0}
  <ProjectsPopup tags={cv.tags} {text} />
  <!-- TOPBAR DIV (POPUP: mobile) -->
  {#if innerWidth < 1200 && sidebarLoaded}
    <Sidebar info={cv.info} bind:sidebarContainer {text} />
  {/if}
  <div class="container-cv" bind:this={containerCv}>
    <!-- SIDEBAR DIV (LEFT: desktop) -->
    {#if innerWidth >= 1200 && sidebarLoaded}
      <Sidebar info={cv.info} {footer} {containerCv} {text} />
    {/if}
    <!-- MOBILE TOP BAR -->
    {#if innerWidth < 1200}
      <div id="topbar" bind:this={topbar}>
        <button
          class={scrollY <= 53
            ? "topbar-button topbar-button-big"
            : "topbar-button"}
          on:click={() => showSidebar(true)}
          bind:this={buttonTopbar}
        >
          {#if scrollY > 53}
            <SvgIcon size="23" path={mdiAccount} type="mdi" />
          {/if}
        </button>
        <button
          class={scrollY <= 53 ? "none" : "topbar-button"}
          on:click={() => (window.location.href = `/lang=${otherlang}`)}
        >
          <span class={`fi fi-${flag} flag-little`}></span>
        </button>
        <h1 class={scrollY <= 53 ? "topbar-name" : "topbar-name-little"}>
          {cv.info.full_name}
        </h1>
      </div>
      <div id="fake-topbar" />
    {/if}
    <!-- MAIN DIV (RIGHT: desktop/CENTER: mobile) -->
    <div class="main">
      {#if innerWidth >= 1200}
        <div class="lang-btn-container">
          <button
            class="footer-btn lang-btn"
            on:click={() => (window.location.href = `/lang=${otherlang}`)}
          >
            <p>{text.lang}</p>
            <span class={`fi fi-${flag} flag-size`}></span>
          </button>
        </div>
        <h1 class="name">{cv.info.full_name}</h1>
      {/if}
      <h2 class="name">{@html cv.info.title}</h2>
      <Section icon={mdiSchool} title={text.education} />
      <SlideShow
        data={cv.education}
        type={Education}
        timeline="true"
        reverse="true"
      />
      <Section icon={mdiBriefcase} title={text.experience} />
      <SlideShow
        data={cv.experience}
        type={Experience}
        timeline="true"
        reverse="true"
      />
      <Section icon={mdiWrench} title={text.projects} />
      <SlideShow
        data={cv.project}
        type={Projects}
        show_max_index={true}
        {text}
      />
      <Section icon={mdiPencil} title={text.skills} />
      {#each cv.skills as skill, index}
        <SubSection
          icon={cv.categories[index].icon}
          title={cv.categories[index].name}
        />
        {#if sidebarLoaded}
          <div class="subsection">
            {#each skill as pilldata, pill_dataindex}
              {#if pilldata.is_shown}
                <Pill
                  bind:this={pillInstances[index][pill_dataindex]}
                  name={pilldata.skill}
                  type_icon={pilldata.type_icon}
                  icon={pilldata.icon}
                  color={pilldata.color}
                  show_tooltip={true}
                  tooltip_data={cv.project_skills[pilldata.id - 1]}
                  {text}
                />
              {/if}
            {/each}
            {#if needMore(skill)}
              <button
                class="pill-container pill-more"
                on:click={() => moreHandler(index)}
              >
                <SvgIcon
                  type="mdi"
                  path={pillShowMore[index] === true
                    ? mdiArrowDown
                    : mdiArrowUp}
                  size="20"
                />
                <p>{text.more}</p>
              </button>
            {/if}
          </div>
          <div class={pillShowMore[index] === true ? "subsection" : "none"}>
            {#each skill as pilldata, pill_dataindex}
              {#if !pilldata.is_shown}
                <Pill
                  bind:this={pillInstances[index][pill_dataindex]}
                  name={pilldata.skill}
                  type_icon={pilldata.type_icon}
                  icon={pilldata.icon}
                  color={pilldata.color}
                  show_tooltip={true}
                  tooltip_data={cv.project_skills[pilldata.id - 1]}
                  {text}
                />
              {/if}
            {/each}
          </div>
        {/if}
      {/each}
      <SubSection icon={mdiEarth} title={text.languages} />
      <div class="subsection flag-container end">
        {#each cv.languages as langdata}
          <FlagComponent
            lang={langdata.lang}
            level={langdata.level}
            icon={langdata.icon_alpha}
          />
        {/each}
      </div>
    </div>
  </div>
  <!-- Footer -->
  <div class="footer" bind:this={footer}>
    <!-- Footer desktop -->
    {#if innerWidth >= 1200}
      <div />
      <div class="footer-text">
        <p>
          {text.madewith}
          <SvgIcon size="20" path={mdiHeart} type="mdi" />
          {text.usingsvelte}
        </p>
        <p>{new Date().getFullYear()} • {text.copyright}</p>
      </div>
      <div class="footer-btn-container">
        <a
          class="footer-btn footer-github"
          href="https://github.com/yoboujon/curriculum-vitae"
          target="_blank"
        >
          <img
            height="30"
            width="30"
            src="https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/github.svg"
            alt="github"
          />
          <p>{text.github}</p>
        </a>
      </div>
      <!-- Footer mobile -->
    {:else}
      <div class="footer-text">
        <p>
          {text.madewith}
          <SvgIcon size="20" path={mdiHeart} type="mdi" />
          {text.usingsvelte}
        </p>
        <p>{new Date().getFullYear()} • {text.copyright}</p>
      </div>
      <div class="footer-mobile-btn">
        <div class="footer-btn-container">
          <a
            class="footer-btn footer-github"
            href="https://github.com/yoboujon/curriculum-vitae"
            target="_blank"
          >
            <img
              height="30"
              width="30"
              src="https://cdn.jsdelivr.net/npm/simple-icons@v9/icons/github.svg"
              alt="github"
            />
            <p>{text.github}</p>
          </a>
        </div>
      </div>
    {/if}
  </div>
{:else}
  <h1 class="h1 text-center">Oops, could not load database :/</h1>
{/if}
