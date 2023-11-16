<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiMapMarker, mdiCog } from "@mdi/js";
    import "$lib/css/education.css";
    import { formatYear } from "$lib/js/date.js"

    export let active = false;
    export let data;

    const title = data.school;
    const location = data.school_location;
    const speciality = data.speciality;
    const option = data.school_options;
    const picture = data.picture_url;
    const end_year = data.end_year === null ? '' : formatYear(data.end_year);
    const start_year = data.start_year === null ? '' : (data.end_year === null ? formatYear(data.start_year) : '-'+formatYear(data.start_year));
</script>

<div class="education-container">
    <div class="education-main {active ? '' : 'education-unactive'}">
        <div class="education-img-container">
            <img class="education-img" src={picture} alt="Education" />
        </div>
        <div class="education-text-container">
            <h1 class="education-title">{title}</h1>
            {#if location}
                <div class="education-subtitle-container">
                    <SvgIcon size="35" path={mdiMapMarker} type="mdi" />
                    <p class="education-subtitle education-aftericon">
                        {location}
                    </p>
                </div>
            {/if}
            {#if speciality}
                <div class="education-subtitle-container">
                    <SvgIcon size="35" path={mdiCog} type="mdi" />
                    <p class="education-subtitle education-aftericon">
                        {speciality}
                    </p>
                </div>
            {/if}
            {#if option}
                <p class="education-subtitle">{option}</p>
            {/if}
        </div>
    </div>
    <div class="education-time">
        <div class="education-bubble" />
        <h2 class="education-date">{`${end_year}${start_year}`}</h2>
    </div>
</div>
