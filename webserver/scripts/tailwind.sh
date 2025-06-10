#!/bin/bash

if [ "$1" = '-h' ]; then
	echo "add -idoc for installation help"
	echo "add -doc for theme docs"
	echo "no flag to compile"

elif [ "$1" = '-idoc' ]; then
	setsid firefox https://v2.tailwindcss.com/docs/installation#using-tailwind-cli 
	echo "check other desktops in multi i3 instances"
elif [ "$1" = '-doc' ]; then
	setsid firefox https://tailwindcss.com/docs/color
	echo "check other desktops in multi i3 instances"
else
	echo "Compiling tailwindcss into /static/css/layout.css"
	npx tailwindcss -c tailwind.config.js -i global.css -o static/css/layout.css --minify
fi


