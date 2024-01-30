<script>
  import { onDestroy } from "svelte";

  import "$lib/css/project-popup.css";
  import "$lib/css/slide.css";
  import Pill from "$lib/components/pill.svelte";
  import { showPopup, actualData, filterTag } from "$lib/js/popup.js";
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
  import { formatMonth } from "$lib/js/date.js";

  // Variables
  const unsubscribe = actualData.subscribe(popupShowed);
  let popupMain;
  let active = false;

  // Informations
  export let tags;

  // Not exported but still Informations
  let filteredTags = [];
  let title = "Title";
  let date = "Date";
  let type_project = "Type of project";
  let picture;
  let description = "Description";
  let report_link;
  let github_link;
  let archive_link;
  let application_link;
  let id = 0;

  async function popupShowed(data) {
    /**
     * Checking if when this function is called data is not equal to 0.
     * It could call the next lines which involve modifying undeclared data
     * or using import statements that cannot be fullfilled because the document
     * is not yet loaded.
     */
    if (data != 0) {
      title = data.title;
      date =
        formatMonth(data.date_done).charAt(0).toUpperCase() +
        formatMonth(data.date_done).slice(1);
      type_project = data.type_project;
      picture = data.picture_name;
      description = data.description;
      id = data.id;
      report_link = data.report_link;
      github_link = data.github_link;
      archive_link = data.archive_link;
      application_link = data.application_link;
      filteredTags = filterTag(tags, id);
      // Active set to true after the await to avoid conflict when clicking outside while the popup hasn't showed yet.
      setTimeout(() => {
        active = true;
      }, 10);
    }
  }

  onDestroy(() => {
    unsubscribe();
  });

  function hidePopup(event) {
    if (!active && popupMain.style.visibility === "visible") {
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
      {#if report_link != null || github_link != null || archive_link != null || application_link != null}
        <div class="slide-subtitle-container">
          <SvgIcon size="35" path={mdiLink} type="mdi" />
          <p class="slide-subtitle slide-aftericon">Links</p>
        </div>
        <div class="project-popup-link-container">
          {#if report_link != null}
            <a
              class="project-popup-download project-popup-report"
              href={report_link}
              target="_blank"
            >
              <SvgIcon size="20" path={mdiFileDocumentOutline} type="mdi" />
              <p>See Report</p>
            </a>
          {/if}
          {#if github_link != null}
            <a
              class="project-popup-download project-popup-github"
              href={github_link}
              target="_blank"
            >
              <SvgIcon size="20" path={mdiGithub} type="mdi" />
              <p>Github Repository</p>
            </a>
          {/if}
          {#if archive_link != null}
            <a
              class="project-popup-download project-popup-archive"
              href={archive_link}
              target="_blank"
            >
              <SvgIcon size="20" path={mdiBookMultiple} type="mdi" />
              <p>Download Archive</p>
            </a>
          {/if}
          {#if application_link != null}
            <a
              class="project-popup-download project-popup-application"
              href={application_link}
              target="_blank"
            >
              <SvgIcon size="20" path={mdiDownload} type="mdi" />
              <p>Download Application</p>
            </a>
          {/if}
        </div>
      {/if}
      <!-- Tags -->
      {#if filteredTags.length != 0}
        <div class="slide-subtitle-container">
          <SvgIcon size="35" path={mdiTag} type="mdi" />
          <p class="slide-subtitle slide-aftericon">Tags</p>
        </div>
        <div class="project-popup-link-container">
          {#each filteredTags as tag}
            <Pill
              name={tag.lang}
              type_icon={tag.type_icon}
              icon={tag.icon}
              color="#F8F1F1"
              shadow_color="#261C2C"
            />
          {/each}
        </div>
      {/if}
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
