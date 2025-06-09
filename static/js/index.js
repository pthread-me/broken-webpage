// Theme and transition controllers



function getTheme() {
  if (typeof localStorage !== 'undefined' && localStorage.getItem('theme') !==null ) {
    return localStorage.getItem('theme');
  }
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 1;
  }
  return 0;
}


const toggleButton = document.getElementById('theme_change'); 
if (toggleButton) {
  toggleButton.addEventListener('click', () => {
		document.getElementsByTagName('html')[0].classList.add('theme-transition');
		document.getElementsByTagName('html')[0].classList.toggle('dark')
		localStorage.setItem('theme', getTheme()^1);
  });
}

