<script>
    import { onMount } from "svelte";
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiCalendarRange, mdiPlus } from "@mdi/js";
    import "$lib/css/slide.css";
    import { formatMonth } from "$lib/js/date.js";

    export let active = false;
    export let data;

    const title = data.title;
    const description = data.description;
    const issued_date = formatMonth(data.date_done).charAt(0).toUpperCase()+formatMonth(data.date_done).slice(1);
    let picture;
    onMount(async () => {
        picture = (await import(`/src/lib/img/${data.picture_name}`)).default;
    });
</script>

<div class="slide-container">
    <div class="slide-main {active ? '' : 'slide-unactive'}">
        <div class="slide-img-container">
            <img class="slide-img" src={picture} alt="projects" />
        </div>
        <div class="slide-text-container">
            <h1 class="slide-title">{title}</h1>
            <div class="slide-subtitle-container">
                <SvgIcon size="35" path={mdiCalendarRange} type="mdi" />
                <p class="slide-subtitle slide-aftericon">
                    {issued_date}
                </p>
            </div>
            <div class="slide-subtitle-container">
                <p class="slide-subtitle slide-text">
                    {description}
                </p>
            </div>
            <div class="slide-button-container">
                <button class="slide-button">
                    <SvgIcon size="20" path={mdiPlus} type="mdi" />
                    More</button
                >
            </div>
        </div>
    </div>
</div>
