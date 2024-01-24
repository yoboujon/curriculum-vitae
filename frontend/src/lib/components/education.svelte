<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiMapMarker, mdiCog } from "@mdi/js";
    import "$lib/css/slide.css";
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

<div class="slide-container">
    <div class="slide-main {active ? '' : 'slide-unactive'}">
        <div class="slide-img-container">
            <img class="slide-img" src={picture} alt="Education" />
        </div>
        <div class="slide-text-container">
            <h1 class="slide-title">{title}</h1>
            <img class="slide-img-mobile" src={picture} alt="Education" />
            {#if location}
                <div class="slide-subtitle-container">
                    <SvgIcon size="35" path={mdiMapMarker} type="mdi" />
                    <p class="slide-subtitle slide-aftericon">
                        {location}
                    </p>
                </div>
            {/if}
            {#if speciality}
                <div class="slide-subtitle-container">
                    <SvgIcon size="35" path={mdiCog} type="mdi" />
                    <p class="slide-subtitle slide-aftericon">
                        {speciality}
                    </p>
                </div>
            {/if}
            {#if option}
                <p class="slide-subtitle">{option}</p>
            {/if}
        </div>
    </div>
    <div class="slide-time">
        <div class="slide-bubble" />
        <h2 class="slide-date">{`${end_year}${start_year}`}</h2>
    </div>
</div>
