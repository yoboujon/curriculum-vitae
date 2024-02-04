export function showSidebar(show) {
  const sidebarContainer = document.getElementById('sidebar-container');
  const body = document.getElementsByTagName('body');

  if (show) {
    body[0].style.overflow = 'hidden';
    sidebarContainer.style.visibility = 'visible';
  } else {
    body[0].style.overflow = '';
    sidebarContainer.style.visibility = 'hidden';
  }
}