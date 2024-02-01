import {writable} from 'svelte/store';

export function showPopup(show, projectId) {
  const background = document.getElementById('project-popup-background');
  const mainPopup = document.getElementById('project-popup-main');
  const body = document.getElementsByTagName('body');

  if (show) {
    body[0].style.overflow = 'hidden';
    background.style.visibility = 'visible';
    mainPopup.style.visibility = 'visible';
  } else {
    body[0].style.overflow = '';
    background.style.visibility = 'hidden';
    mainPopup.style.visibility = 'hidden';
  }
  if (projectId != null) {
    actualData.update(() => {
      let popup;
      const unsubscribe = popupDatas.subscribe(value => {
        popup = value;
      });
      for (const p of popup) {
        if (p.id == projectId) {
          unsubscribe();
          return p;
        }
      }
      unsubscribe();
    });
  }
}

export function filterTag(tags, id)
{
  let returnTags = [];
  for(const tag of tags)
  {
    if(tag.project_id == id)
    {
      returnTags.push(tag);
    }
  }
  return returnTags;
}

export let popupDatas = writable([]);
export let actualData = writable(0);