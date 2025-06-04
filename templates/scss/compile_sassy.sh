#!/bin/bash

suffix=".css"

for f in ./stylesheets/*.scss; do
	[ -e $f ] | continue	# check that file exists
	new_file=$(basename $f .scss)$suffix  # get the name without the .scss suffix
	sassc $f ../assets/css/$new_file # call the scss compiler on the .scss file to prod a .css
done
