<script>
    import SvgIcon from "@jamescoyle/svelte-icon";
    import { mdiArrowRight, mdiChevronLeft } from "@mdi/js";
    import "$lib/css/slideshow.css"

    export let data = [];
    export let type;

    let slideshow_index = 0;
    let slideshow_hidden = [];
    function slideEducation() {
        const slideshowElements = document.querySelectorAll(".education-main");
        if (slideshow_index >= data.length - 1) {
            slideshow_hidden = [];
            slideshow_index = 0;
        } else {
            slideshow_hidden.push(slideshow_index);
            slideshow_index++;
        }
        let transformValue = 0;
        for (const id of slideshow_hidden) {
                transformValue += (slideshowElements[id].clientWidth+(5*16));
            }
        slideshowElements.forEach((element, id) => {
            let newtransformValue = transformValue;
            if(slideshow_hidden.includes(id)) {
                newtransformValue*=1.1;
            }
            element.style.transform = `translateX(-${newtransformValue}px)`;
        });
    }
</script>

<div class="slideshow">
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
