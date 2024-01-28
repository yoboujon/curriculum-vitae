<script>
  import { onDestroy } from "svelte";

  import "$lib/css/project-popup.css";
  import "$lib/css/slide.css";
  import Pill from "$lib/components/pill.svelte";
  import { showPopup, actualData } from "$lib/js/popup.js";
  import SvgIcon from "@jamescoyle/svelte-icon";
  import {
    mdiClose,
    mdiSchool,
    mdiAccount,
    mdiCalendarRange,
    mdiTextBox,
    mdiFileDocumentOutline,
    mdiGithub,
    mdiLink,
    mdiTag,
    mdiBookMultiple,
    mdiDownload,
  } from "@mdi/js";

  // Variables
  const unsubscribe = actualData.subscribe(popupShowed);
  let popupMain;
  let active = false;

  // Informations
  export let tags;
  let title = "Title";
  let date = "Date";
  let type_project = "Type of project";
  let picture;
  let description = "Description";
  let id = 0;

  async function popupShowed(data) {
    /**
     * Checking if when this function is called data is not equal to 0.
     * It could call the next lines which involve modifying undeclared data
     * or using import statements that cannot be fullfilled because the document
     * is not yet loaded.
     */
    if (data != 0) {
      const superData = data;
      title = superData.title;
      date = data.date;
      type_project = data.type_project;
      picture = (await import(`/src/lib/img/${data.picture_name}`)).default;
      description = data.description;
      id = data.id;
      // Active set to true after the await to avoid conflict when clicking outside while the popup hasn't showed yet.
      active = true;
    }
  }

  onDestroy(() => {
    unsubscribe();
  });

  function hidePopup(event) {
    if (!active) {
      return;
    }

    const x = event.clientX;
    const popupXmin = popupMain.offsetLeft;
    const popupXmax = popupXmin + popupMain.offsetWidth;

    const y = event.clientY;
    const popupYmin = popupMain.offsetTop;
    const popupYmax = popupYmin + popupMain.offsetHeight;

    if (
      !(x >= popupXmin && x <= popupXmax && y >= popupYmin && y <= popupYmax)
    ) {
      active = false;
      showPopup(false, null);
    }
  }
</script>

<svelte:window on:click={hidePopup} />

<div id="project-popup-main" bind:this={popupMain}>
  <!--Closing button-->
  <button
    class="project-popup-close"
    on:click={() => {
      active = false;
      showPopup(false, null);
    }}
  >
    <SvgIcon size="25" path={mdiClose} type="mdi" />
  </button>

  <div class="project-popup-container">
    <!-- Information -->
    <div class="prout">
      <h1 class="slide-title">{title}</h1>
      <div class="project-popup-img-container">
        <img class="project-popup-img" src={picture} alt="Project Popup" />
      </div>
      <!-- Subinfo (date, type...) -->
      <div class="project-popup-subinfo-container">
        <div class="slide-subtitle-container">
          <SvgIcon size="35" path={mdiCalendarRange} type="mdi" />
          <p class="project-popup-subtitle slide-aftericon">{date}</p>
        </div>
        <div class="slide-subtitle-container">
          <SvgIcon
            size="35"
            path={type_project == "School" ? mdiSchool : mdiAccount}
            type="mdi"
          />
          <p class="project-popup-subtitle slide-aftericon">{type_project}</p>
        </div>
      </div>
      <!-- Links -->
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiLink} type="mdi" />
        <p class="slide-subtitle slide-aftericon">Links</p>
      </div>
      <div class="project-popup-link-container">
        <button class="project-popup-download project-popup-report">
          <SvgIcon size="20" path={mdiFileDocumentOutline} type="mdi" />
          <p>See Report</p>
        </button>
        <button class="project-popup-download project-popup-github">
          <SvgIcon size="20" path={mdiGithub} type="mdi" />
          <p>Github Repository</p>
        </button>
        <button class="project-popup-download project-popup-archive">
          <SvgIcon size="20" path={mdiBookMultiple} type="mdi" />
          <p>Download Archive</p>
        </button>
        <button class="project-popup-download project-popup-application">
          <SvgIcon size="20" path={mdiDownload} type="mdi" />
          <p>Download Application</p>
        </button>
      </div>
      <!-- Tags -->
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiTag} type="mdi" />
        <p class="slide-subtitle slide-aftericon">Tags</p>
      </div>
      <div class="project-popup-link-container">
        {#each tags as tag}
          {#if tag.project_id === id}
            <Pill
              name={tag.lang}
              type_icon={tag.type_icon}
              icon={tag.icon}
              color="#F8F1F1"
              shadow_color="#261C2C"
            />
          {/if}
        {/each}
      </div>
    </div>
    <!-- Text -->
    <div>
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiTextBox} type="mdi" />
        <p class="slide-subtitle slide-aftericon">Description</p>
      </div>
      <p class="slide-subtitle project-popup-text">{description}</p>
    </div>
  </div>
</div>
<div id="project-popup-background"></div>
