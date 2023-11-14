<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiMapMarker, mdiCog } from "@mdi/js";
    import "$lib/css/education.css";

    export let active = false;
    export let data;
    export let picture =
        "https://bib.insa-toulouse.fr/_resource/info_pratiques/INSA-Venir-Bib-photo-8_0.jpg";

    const title = data.school;
    const location = data.school_location;
    const speciality = data.speciality;
    const option = data.school_options;
</script>

<div class="education-main {active ? '' : 'education-unactive'}">
    <div class="education-img-container">
        <img class="education-img" src={picture} alt="Education" />
    </div>
    <div class="education-text-container">
        <h1 class="education-title">{title}</h1>
        {#if location}
            <div class="education-subtitle-container">
                <SvgIcon size="35" path={mdiMapMarker} type="mdi" />
                <p class="education-subtitle education-aftericon">{location}</p>
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
