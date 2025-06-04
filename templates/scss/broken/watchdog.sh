#!/bin/bash

touch logs

while inotifywait '@./watchdog.sh' '@./compile_sassy.sh' '@./logs' \
	-e modify -m -r stylesheets  -t 900 --timefmt "%c" \
	--format "%e was tiggered at %T in %w file %f"  -o \
	logs; do
	echo "hmm\n"
	./compile_sassy.sh
done
