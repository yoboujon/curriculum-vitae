<script>
  import SvgIcon from "@jamescoyle/svelte-icon";
  import { mdiArrowRight, mdiChevronLeft } from "@mdi/js";
  import "$lib/css/slideshow.css";
  import { onMount } from "svelte";
  import {
    createTimeLine,
    updateTimeLine,
    slideContainerCount,
    slideTimelineCount,
    slideStringCount,
  } from "$lib/js/slideshow.js";

  // Exported values
  export let data = [];
  export let type;
  export let timeline = false;
  export let reverse = false;
  if (reverse) {
    data = data.reverse();
  }

  // Resizing variables
  let resizing = false;

  // Slideshow global variables
  let slideshow_index = 0;
  let slideshow_hidden = [];

  let slideshowTimeline;
  let slideshowElements;
  let slideshowBubbles;

  // Timeline global variables
  let slideshow;
  let bubbles = [];
  onMount(() => {
    /*    Slideshow    */
    //global writer count
    const currentSlideCount = $slideContainerCount;
    // Sliced array from currentSlideCount to data.length
    slideshowElements = Array.from(
      document.querySelectorAll(".slide-container"),
    ).slice(currentSlideCount, currentSlideCount + data.length);
    // Updating with the current length
    slideContainerCount.update((value) => {
      return value + data.length;
    });

    if (timeline) {
      /*    SlideTimeline    */
      //global writer count
      const currentTimelineCount = $slideTimelineCount;
      // Creating the string between the bubbles
      slideshowBubbles = Array.from(
        document.querySelectorAll(".slide-bubble"),
      ).slice(currentTimelineCount, currentTimelineCount + data.length);

      // Creating strings
      for (const element of slideshowBubbles) {
        bubbles.push({
          left: element.offsetLeft,
          top: element.offsetTop,
        });
      }

      createTimelineStrings();
    }
  });

  function slideCards() {
    // Set or reset slideshow index, hidden array and timeling.
    if (slideshow_index >= data.length - 1) {
      resetSlideCards();
    } else {
      slideshow_hidden.push(slideshow_index);
      slideshow_index++;
    }

    // Incrementing the transformValue for each element
    let transformValue = 0;
    for (const id of slideshow_hidden) {
      transformValue += slideshowElements[id].clientWidth;
    }

    // Translating elements
    slideshowElements.forEach((element, id) => {
      /* Slideshow translating*/
      let newtransformValue = transformValue;
      if (slideshow_hidden.includes(id)) {
        // 1.1 because when in 'unactive' state, the scale is 0.9, adjusting the actual ratio
        newtransformValue *= 1.1;
      }
      element.style.transform = `translateX(-${newtransformValue}px)`;

      /* Slideshow Timeline translating */
      if (timeline) {
        if (slideshowTimeline[id] != undefined) {
          slideshowTimeline[id].style.transform =
            `translateX(-${transformValue}px)`;
          if (slideshow_hidden.includes(id)) {
            slideshowTimeline[id].style.backgroundColor =
              "var(--color-background)";
          }
        }
      }
    });
  }

  function resetSlideCards() {
    return new Promise((resolve) => {
      slideshow_hidden = [];
      slideshow_index = 0;
      for (const element of slideshowElements) {
        element.style.transform = "";
      }
      if (timeline) {
        for (const timeline of slideshowTimeline) {
          timeline.style.backgroundColor = "";
          timeline.style.transform = "";
        }
      }

      resolve();
    });
  }

  function createTimelineStrings() {
    const stringTimelineElements = createTimeLine(bubbles);
    for (const div of stringTimelineElements) {
      slideshow.appendChild(div);
    }
    // Updating with the current length
    slideTimelineCount.update((value) => {
      return value + data.length;
    });

    //global writer count
    const currentStringCount = $slideStringCount;
    /* Sliced array from currentTimelineCount to data.length
        Everytime a slideshow created the slide string, the queryselector all will get all the elements
        We do not want that and only want the slide-string we are interested in
        Starting from the oldest count, finishing by our actual length */
    slideshowTimeline = Array.from(
      document.querySelectorAll(".slide-string"),
    ).slice(
      currentStringCount,
      currentStringCount + stringTimelineElements.length,
    );
    // Updating with the current length
    slideStringCount.update((value) => {
      return value + stringTimelineElements.length;
    });
  }

  async function changeSize() {
    if (timeline && !resizing) {
      resizing = true;

      await resetSlideCards();
      await new Promise(resolve => setTimeout(resolve, 400));
      updateTimeLine(slideshowTimeline, slideshowBubbles);
      resizing = false;
      //global writer count
    }
  }
</script>

<svelte:window on:resize={changeSize} />

<div class="slideshow" bind:this={slideshow}>
  <button class="slideshow_btn" on:click={slideCards}>
    <div>
      <SvgIcon
        size="45"
        path={slideshow_index >= data.length - 1
          ? mdiChevronLeft
          : mdiArrowRight}
        type="mdi"
      />
    </div>
  </button>
  {#each data as selected_data, index (index)}
    <svelte:component
      this={type}
      data={selected_data}
      active={index == slideshow_index ? true : false}
    />
  {/each}
</div>
