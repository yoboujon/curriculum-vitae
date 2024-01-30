<script>
  import { onMount } from "svelte";
  import SvgIcon from "@jamescoyle/svelte-icon";
  import { mdiCalendarRange, mdiPlus, mdiAccount, mdiSchool } from "@mdi/js";
  import "$lib/css/slide.css";
  import { formatMonth } from "$lib/js/date.js";
  import { showPopup, popupDatas } from "$lib/js/popup.js";

  export let active = false;
  export let data;

  const title = data.title;
  const description = data.description;
  const issued_date =
    formatMonth(data.date_done).charAt(0).toUpperCase() +
    formatMonth(data.date_done).slice(1);
  const project_type = data.type_project;
  let picture;

  onMount(async () => {
    picture = (await import(`/src/lib/img/${data.picture_name}`)).default;

    popupDatas.update((value) => {
      value.push(data);
      return value;
    });
  });
</script>

<div class="slide-container">
  <div class="slide-more slide-main {active ? '' : 'slide-unactive'}">
    <div class="slide-img-container">
      <img class="slide-img" src={picture} alt="Projects" />
    </div>
    <div class="slide-text-container">
      <h1 class="slide-title">{title}</h1>
      <div class="slide-img-mobile-container">
        <img class="slide-img-mobile" src={picture} alt="Projects" />
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
          {description}
        </p>
      </div>
      <div class="slide-button-container">
        <button class="slide-button" on:click={() => showPopup(true, data.id)}>
          <SvgIcon size="20" path={mdiPlus} type="mdi" />
          More</button
        >
      </div>
    </div>
  </div>
</div>
