import { writable } from 'svelte/store';

export function createTimeLine(positionsArray) {
  let divArray = [];
  for (let i = 0; i < positionsArray.length - 1; i++) {
    var newDiv = document.createElement('div');
    newDiv.className = "slide-string";
    const left = positionsArray[i].left + (2.5 * 16);
    newDiv.style.left = `${left}px`;
    const top = positionsArray[i].top + 16;
    newDiv.style.top = `${top}px`;
    const width = ((positionsArray[i + 1].left) - (positionsArray[i].left));
    newDiv.style.width = `${width}px`;
    divArray.push(newDiv);
  }
  return divArray;
}

export let slideContainerCount = writable(0);
export let slideTimelineCount = writable(0);
export let slideStringCount = writable(0);