<script>
  import { onMount } from "svelte";
  import "$lib/css/suggest.css";
  import "$lib/css/project-popup.css";
  import "$lib/css/cv.css";

  import SvgIcon from "@jamescoyle/svelte-icon";
  import { mdiFileDocumentOutline, mdiClose } from "@mdi/js";

  export let url;
  export let text;
  let hide = false;
  let suggestMain;
  let innerWidth;

  onMount(async () => {
    await new Promise((resolve) => setTimeout(resolve, 5000));
    suggestMain.style.opacity = `${1}`;
  });
</script>

<svelte:window bind:innerWidth />

<div class={hide ? "suggest-main none" : "suggest-main"} bind:this={suggestMain}>
  <div class="suggest-close">
    <button
      on:click={() => {
        hide = true;
      }}
    >
      <SvgIcon size="25" path={mdiClose} type="mdi" />
    </button>
  </div>
  <div class="suggest-main-container">
    <div class="suggest-container">
      <div>
        <h1 class="suggest-h1">{text.suggest_title}</h1>
        <p class="suggest-text">
          {text.suggest_subtext}
        </p>
      </div>
    </div>
    <div class="suggest-container suggest-center">
      <a
        class="project-popup-download project-popup-report suggest-download"
        href={url}
        target="_blank"
      >
        <SvgIcon
          size={innerWidth < 1000 ? "15" : "20"}
          path={mdiFileDocumentOutline}
          type="mdi"
        />
        <span>{text.suggest_button}</span></a
      >
    </div>
  </div>
</div>
