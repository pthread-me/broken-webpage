function getTheme() {
  if (typeof localStorage !== 'undefined' && localStorage.getItem('theme')) {
    return localStorage.getItem('theme');
  }
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 'dark';
  }
  return 'light';
}

function applyTheme(theme) {
  if (theme === 'dark') {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
}

window.addEventListener('load', () => {
  const theme = getTheme();
  applyTheme(theme);
});

const toggleButton = document.getElementById('theme_change'); 
if (toggleButton) {
  toggleButton.addEventListener('click', () => {
		document.getElementsByTagName('html')[0].classList.toggle('dark')
		localStorage.setItem('theme', getTheme());
  });
}
