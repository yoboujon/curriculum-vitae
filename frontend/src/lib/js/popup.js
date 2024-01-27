import {writable} from 'svelte/store';

export function showPopup(show, popupObject) {
  const background = document.getElementById('project-popup-background');
  const mainPopup = document.getElementById('project-popup-main');
  const body = document.getElementsByTagName('body');

  if (show === true) {
    body[0].style.overflow = 'hidden';
    background.style.visibility = 'visible';
    mainPopup.style.visibility = 'visible';
  } else {
    body[0].style.overflow = '';
    background.style.visibility = 'hidden';
    mainPopup.style.visibility = 'hidden';
  }
  if (popupObject != null) {
    popupDatas.update(() => {
      return popupObject;
    });
  }
}

export let popupDatas = writable(0);