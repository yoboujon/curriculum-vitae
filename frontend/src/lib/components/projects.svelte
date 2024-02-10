<script>
  import { onMount } from "svelte";
  import SvgIcon from "@jamescoyle/svelte-icon";
  import {
    mdiCalendarRange,
    mdiBookOpenVariant,
    mdiAccount,
    mdiSchool,
  } from "@mdi/js";
  import "$lib/css/slide.css";
  import { formatMonth } from "$lib/js/date.js";
  import { showPopup, popupDatas } from "$lib/js/popup.js";

  export let active = false;
  export let text;
  export let data;
  export let max = 0;
  export let actualnum = 0;

  const title = data.title;
  const short_description = data.short_description;
  const issued_date =
    formatMonth(data.date_done).charAt(0).toUpperCase() +
    formatMonth(data.date_done).slice(1);
  const project_type = data.type_project;

  onMount(async () => {
    popupDatas.update((value) => {
      value.push(data);
      return value;
    });
  });
</script>

<div class="slide-container">
  <div class="slide-more slide-main {active ? '' : 'slide-unactive'}">
    <div class="slide-img-container">
      <img class="slide-img" src={data.picture_name} alt="Projects" />
    </div>
    <div class="slide-text-container">
      <h1 class="slide-title slide-overflow-title">{title}</h1>
      <div class="slide-img-mobile-container">
        <img class="slide-img-mobile" src={data.picture_name} alt="Projects" />
      </div>
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiCalendarRange} type="mdi" />
        <p class="slide-subtitle slide-aftericon">
          {issued_date}
        </p>
      </div>
      <div class="slide-subtitle-container">
        <SvgIcon
          size="35"
          path={project_type == "School" ? mdiSchool : mdiAccount}
          type="mdi"
        />
        <p class="slide-subtitle slide-aftericon">
          {project_type}
        </p>
      </div>
      <div class="slide-subtitle-container">
        <p class="slide-subtitle slide-text slide-overflow slide-justify">
          {short_description}
        </p>
      </div>
      <div class="slide-count-container">
        <div class="slide-count">
          {`${actualnum}/${max}`}
        </div>
      </div>
      <div class="slide-button-container">
        <button class="slide-button" on:click={() => showPopup(true, data.id)}>
          <SvgIcon size="20" path={mdiBookOpenVariant} type="mdi" />
          <p class="slide-aftericon slide-button-text">{text.projects_read}</p>
        </button>
      </div>
    </div>
  </div>
</div>
