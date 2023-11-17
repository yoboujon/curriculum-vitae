<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiCalendarRange, mdiPlus } from "@mdi/js";
    import "$lib/css/projects.css";

    export let active = false;
    export let data;

    const title = data.title;
    const description = data.description;
    const issued_date = "2023";
    const picture = data.picture_url;
</script>

<div class="projects-container">
    <div class="projects-main {active ? '' : 'projects-unactive'}">
        <div class="projects-img-container">
            <img class="projects-img" src={picture} alt="projects" />
        </div>
        <div class="projects-text-container">
            <h1 class="projects-title">{title}</h1>
            <div class="projects-subtitle-container">
                <SvgIcon size="35" path={mdiCalendarRange} type="mdi" />
                <p class="projects-subtitle projects-aftericon">
                    {issued_date}
                </p>
            </div>
            <div class="projects-subtitle-container">
                <p class="projects-subtitle projects-text">
                    {description}
                </p>
            </div>
            <div class="projects-button-container">
                <button class="projects-more-button">
                    <SvgIcon size="20" path={mdiPlus} type="mdi" />
                    More</button
                >
            </div>
        </div>
    </div>
</div>
