<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiArrowRight, mdiChevronLeft } from "@mdi/js";
    import "$lib/css/slideshow.css";
    import { onMount } from "svelte";
    import { createTimeLine } from "$lib/js/timeline.js";

    // Exported values
    export let data = [];
    export let type;

    // Slideshow global variables
    let slideshow_index = 0;
    let slideshow_hidden = [];

    // Timeline global variables
    let slideshow;
    let bubbles = [];
    onMount(() => {
        for (const element of document.getElementsByClassName(
            "education-bubble"
        )) {
            bubbles.push({
                left: element.offsetLeft,
                top: element.offsetTop,
            });
        }
        for (const div of createTimeLine(bubbles)) {
            slideshow.appendChild(div);
        }
    });

    function slideEducation() {
        const slideshowElements = document.querySelectorAll(
            ".education-container"
        );
        const slideshowTimeline =
            document.querySelectorAll(".education-string");

        if (slideshow_index >= data.length - 1) {
            slideshow_hidden = [];
            slideshow_index = 0;
            for(const timeline of slideshowTimeline) {
                timeline.style.backgroundColor = '';
            }
        } else {
            slideshow_hidden.push(slideshow_index);
            slideshow_index++;
        }
        let transformValue = 0;
        for (const id of slideshow_hidden) {
            transformValue += slideshowElements[id].clientWidth;
        }
        slideshowElements.forEach((element, id) => {
            let newtransformValue = transformValue;
            if (slideshow_hidden.includes(id)) {
                // 1.1 because when in 'unactive' state, the scale is 0.9, adjusting the actual ratio
                newtransformValue *= 1.1;
            }
            element.style.transform = `translateX(-${newtransformValue}px)`;
            if(slideshowTimeline[id] != undefined) {
                slideshowTimeline[id].style.transform = `translateX(-${transformValue}px)`;
                if (slideshow_hidden.includes(id)) {
                    slideshowTimeline[id].style.backgroundColor = 'var(--color-background)';
                }
            }
        });
    }
</script>

<div class="slideshow" bind:this={slideshow}>
    <button class="slideshow_btn" on:click={slideEducation}>
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
    {#each data.reverse() as selected_data, index (index)}
        <svelte:component
            this={type}
            data={selected_data}
            active={index == slideshow_index ? true : false}
        />
    {/each}
</div>
