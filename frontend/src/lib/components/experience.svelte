<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiMapMarker, mdiCardText, mdiOfficeBuilding } from "@mdi/js";
    import "$lib/css/experience.css";
    import { formatDate } from "$lib/js/date.js";

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
    const picture = data.picture_url;
</script>

<div class="experience-container">
    <div class="experience-main {active ? '' : 'experience-unactive'}">
        <div class="experience-img-container">
            <img class="experience-img" src={picture} alt="Experience" />
        </div>
        <div class="experience-text-container">
            <h1 class="experience-title">{position}</h1>
            <div class="experience-subtitle-container">
                <SvgIcon size="35" path={mdiOfficeBuilding} type="mdi" />
                <p class="experience-subtitle experience-aftericon">
                    {enterprise}
                </p>
            </div>
            {#if location}
                <div class="experience-subtitle-container">
                    <SvgIcon size="35" path={mdiMapMarker} type="mdi" />
                    <p class="experience-subtitle experience-aftericon">
                        {location}
                    </p>
                </div>
            {/if}
            {#if description}
                <div class="experience-subtitle-container">
                    <p class="experience-subtitle">
                        {description}
                    </p>
                </div>
            {/if}
        </div>
    </div>
    <div class="experience-time">
        <div class="experience-bubble" />
        <h2 class="experience-date">{`${end_year}${start_year}`}</h2>
    </div>
</div>
