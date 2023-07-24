#!/bin/sh

echo -e '\n\033[1m==== TODO ====\033[0m'
grep --colour=always -r "TODO" src/

echo -e '\n\033[1m==== FIXME ====\033[0m'
grep --colour=always -r "FIXME" src/
