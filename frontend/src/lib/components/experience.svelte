<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import { mdiMapMarker, mdiOfficeBuilding } from "@mdi/js";
  import "$lib/css/slide.css";
  import { formatDate,formatDateMobile } from "$lib/js/date.js";

  export let active = false;
  export let data;

  const enterprise = data.enterprise;
  const location = data.enterprise_location;
  const description = data.job_description;
  const position = data.job_position;
  const end_year = data.end_year === null ? "" : formatDate(data.end_year);
  const start_year =
    data.start_year === null
      ? ""
      : data.end_year === null
        ? formatDate(data.start_year)
        : " - " + formatDate(data.start_year);
  const mobile_end_year =
    data.end_year === null ? "" : formatDateMobile(data.end_year);
  const mobile_start_year =
    data.start_year === null
      ? ""
      : data.end_year === null
        ? formatDateMobile(data.start_year)
        : "-" + formatDateMobile(data.start_year);
    const picture = data.picture_url;

  // mobile version
  let windowWidth = 0;
</script>

<svelte:window bind:innerWidth={windowWidth} />

<div class="slide-container">
  <div class="slide-main {active ? '' : 'slide-unactive'}">
    <div class="slide-img-container">
      <img class="slide-img" src={picture} alt="Experience" />
    </div>
    <div class="slide-text-container">
      <h1 class="slide-title slide-overflow-title">{position}</h1>
      <div class="slide-img-mobile-container">
        <img class="slide-img-mobile" src={picture} alt="Experience" />
      </div>
      <div class="slide-subtitle-container">
        <SvgIcon size="35" path={mdiOfficeBuilding} type="mdi" />
        <p class="slide-subtitle slide-aftericon">
          {enterprise}
        </p>
      </div>
      {#if location}
        <div class="slide-subtitle-container">
          <SvgIcon size="35" path={mdiMapMarker} type="mdi" />
          <p class="slide-subtitle slide-aftericon">
            {location}
          </p>
        </div>
      {/if}
      {#if description}
        <div class="slide-subtitle-container">
          <p class="slide-subtitle">
            {description}
          </p>
        </div>
      {/if}
    </div>
  </div>
  <div class="slide-time">
    <div class="slide-bubble" />
    <h2 class="slide-date">{`${windowWidth > 1000 ? end_year : mobile_end_year}${windowWidth > 1000 ? start_year : mobile_start_year}`}</h2>
  </div>
</div>
