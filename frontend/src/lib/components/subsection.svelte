<script>
    import "$lib/css/section.css";
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiHelpCircle } from "@mdi/js";

    export let title = "Title";
    export let icon = mdiHelpCircle;
</script>

<div class="subsection-container">
    <SvgIcon size="40" path={icon} type="mdi" />
    <h1 class="section-h2">{title}</h1>
</div>
