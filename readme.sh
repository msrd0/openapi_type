#!/bin/busybox ash
set -o pipefail

check=n
if [ "$1" == "--check" ]
then
	check=y
fi

set -u

sed -i -e 's,#!\[doc = r##",/*!,' -e 's,"##\],*/ //doc,' src/lib.rs
ok=0
for readme in README crates-io
do
	output=$readme.md
	[ $check == y ] && output=$(mktemp)
	cargo readme -t $readme.tpl -o $output
	if [ $check == y ]
	then
		diff $readme.md $output || ok=1
		rm $output
	fi
done
sed -i -e 's,/\*!,#![doc = r##",' -e 's,\*/ //doc,"##],' src/lib.rs
exit $ok
