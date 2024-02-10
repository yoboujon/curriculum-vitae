<script>
    import "/node_modules/flag-icons/css/flag-icons.min.css";
    import "$lib/css/flag.css";

    export let lang;
    export let level;
    export let icon;
</script>

<div class="flag-main">
    <div class="flag-img">
        <span class={`fi fi-${icon} flag-size`}></span>
    </div>
    <div class="flag-text">
        <p>{lang}</p>
        <p>{level}</p>
    </div>
</div>
