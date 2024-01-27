<script>
  import { onDestroy } from "svelte";

  import "$lib/css/project-popup.css";
  import "$lib/css/slide.css";
  import { showPopup, popupDatas } from "$lib/js/popup.js";
  import SvgIcon from "@jamescoyle/svelte-icon";
  import {
    mdiClose,
    mdiSchool,
    mdiAccount,
    mdiCalendarRange,
    mdiTextBox,
  } from "@mdi/js";

  const unsubscribe = popupDatas.subscribe(popupShowed);
  let title = "Title";
  let date = "Date";
  let type_project = "Type of project";
  let picture;
  let description = "Description";

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
    }
  }

  onDestroy(() => {
    unsubscribe();
  });
</script>

<div id="project-popup-main">
  <!--Closing button-->
  <button class="project-popup-close" on:click={() => showPopup(false, null)}>
    <SvgIcon size="25" path={mdiClose} type="mdi" />
  </button>

  <div class="project-popup-container">
    <!-- Information -->
    <div class="project-popup-info-container">
      <h1 class="slide-title">{title}</h1>
      <div class="project-popup-img-container">
        <img class="project-popup-img" src={picture} alt="Project Popup" />
      </div>
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiCalendarRange} type="mdi" />
        <p class="slide-subtitle slide-aftericon">{date}</p>
      </div>
      <div class="slide-subtitle-container">
        <SvgIcon
          size="35"
          path={type_project == "School" ? mdiSchool : mdiAccount}
          type="mdi"
        />
        <p class="slide-subtitle slide-aftericon">{type_project}</p>
      </div>
    </div>
    <!-- Text -->
    <div class="project-popup-text-container">
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiTextBox} type="mdi" />
        <p class="slide-subtitle slide-aftericon">Description</p>
      </div>
      <p class="slide-subtitle project-popup-text">{description}</p>
    </div>
  </div>
</div>
<div id="project-popup-background"></div>
