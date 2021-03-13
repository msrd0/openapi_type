#!/bin/busybox ash
set -eo pipefail

check=n
if [ "$1" == "--check" ]
then
	check=y
fi

set -u

for readme in README crates-io
do
	output=$readme.md
	[ $check == y ] && output=$(mktemp)
	cargo readme -t $readme.tpl -o $output
	if [ $check == y ]
	then
		diff $readme.md $output
		rm $output
	fi
done
