#!/bin/sh
if [ "$1" = "-h" ]; then
   echo "Usage: todos.sh [-u]"
   echo "  -u: point out .unwrap() calls"
   exit 0
fi

echo -e '\033[1m==== TODO ====\033[0m'
grep --colour=always -n -r "TODO" src/

echo -e '\n\033[1m==== FIXME ====\033[0m'
grep --colour=always -n -r "FIXME" src/

echo -e '\n\033[1m==== todo! ====\033[0m'
grep --colour=always -n -r "todo!" src/

if [ "$1" = "-u" ]; then
   echo -e '\n\033[1m==== UNWRAPS ====\033[0m'
   grep --colour=always -n -r ".unwrap()" src/
fi
