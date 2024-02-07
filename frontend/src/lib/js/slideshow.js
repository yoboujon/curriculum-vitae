import {writable} from 'svelte/store';
const mobileString = 24;
const dekstopString = 48;

export function createTimeLine(positionsArray) {
  const isMobile = (positionsArray[0].width < 80);
  const offset = (positionsArray[0].width) / 2;
  let divArray = [];

  for (let i = 0; i < positionsArray.length - 1; i++) {
    var newDiv = document.createElement('div');
    newDiv.className = 'slide-string';
    const left = positionsArray[i].left + offset;
    newDiv.style.left = `${left}px`;
    const top = positionsArray[i].top +
        (offset - (isMobile ? mobileString / 2 : dekstopString / 2));
    newDiv.style.top = `${top}px`;
    const width = ((positionsArray[i + 1].left) - (positionsArray[i].left));
    newDiv.style.width = `${width}px`;
    divArray.push(newDiv);
  }
  return divArray;
}

export function updateTimeLine(slideStringArray, positionsArray) {
  const isMobile = (positionsArray[0].width < 80);
  const offset = (positionsArray[0].width) / 2;

  for (let i = 0; i < positionsArray.length - 1; i++) {
    const left = positionsArray[i].offsetLeft + offset;
    const top = positionsArray[i].offsetTop +
        (offset - (isMobile ? mobileString / 2 : dekstopString / 2));
    const width = ((positionsArray[i + 1].left) - (positionsArray[i].left));

    slideStringArray[i].style.left = `${left}px`;
    slideStringArray[i].style.top = `${top}px`;
    slideStringArray[i].style.width = `${width}px`;
  }
}

export let slideContainerCount = writable(0);
export let slideTimelineCount = writable(0);
export let slideStringCount = writable(0);