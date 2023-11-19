<script>
    import Flag from "$lib/vendor/svelte-flagpack";
    import "$lib/css/flag.css";

    export let lang;
    export let level;
    export let icon;
</script>

<div class="flag-main">
    <div class="flag-img">
        <Flag code={icon} size="l" hasDropShadow />
    </div>
    <div class="flag-text">
        <p>{lang}</p>
        <p><span class="flag-bold">Level: </span>{level}</p>
    </div>
</div>
