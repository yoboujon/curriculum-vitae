<script>
    import "$lib/css/education.css";

    export let active = false;
    export let data;
    export let picture = "https://bib.insa-toulouse.fr/_resource/info_pratiques/INSA-Venir-Bib-photo-8_0.jpg";

    const title = data.school;
    const location = data.school_location;
    const option = data.school_options;
</script>

<div class="education-main {active ? '' : 'education-unactive'}">
    <div class="education-img-container">
        <img class="education-img" src={picture} alt="Education"/>
    </div>
    <div class="education-text-container">
        <h1 class="education-title">{title}</h1>
        <p class="education-subtitle">{location}</p>
        <p class="education-subtitle">{option}</p>
    </div>
</div>
