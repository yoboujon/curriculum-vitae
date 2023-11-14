<script>
    import "$lib/css/sidebar-component.css";
    import SvgIcon from '@jamescoyle/svelte-icon'
    import { mdiHelpCircle } from '@mdi/js';

    export let icon = mdiHelpCircle;
    export let title = "";
    export let description = "";
    const descriptionArray = description.split(";");
</script>

<div class="sc-container">
    <div class="sc-icon">
        <SvgIcon size="45" path={icon} type="mdi"/>
    </div>
    <h1 class="text sc-title">{title}</h1>
    {#each descriptionArray as text}
        <h2 class="text sc-description">{text}</h2>
    {/each}
</div>