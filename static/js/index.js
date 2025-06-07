// Man fuck JS
// if only you knew the amount of time it took to debug this shit
// give me types any mfing day of the week :(

function getTheme() {
  if (typeof localStorage !== 'undefined' && localStorage.getItem('theme') !==null ) {
    return localStorage.getItem('theme');
  }
  if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    return 1;
  }
  return 0;
}

document.addEventListener('DOMContentLoaded', () => {
  console.log("dom loaded")
	const theme = getTheme();
	console.log("theme found was " + theme)
	if (theme == '0')	{
		document.getElementsByTagName('html')[0].classList.toggle('dark');
		localStorage.setItem('theme', '0')
	}else{
		localStorage.setItem('theme', '1')
	}
});


const toggleButton = document.getElementById('theme_change'); 
if (toggleButton) {
  toggleButton.addEventListener('click', () => {
		document.getElementsByTagName('html')[0].classList.toggle('dark')
		localStorage.setItem('theme', getTheme()^1);
  });
}
