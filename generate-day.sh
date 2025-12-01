#!/usr/bin/env sh
cd "$(dirname $(readlink -f "$0") )"

PreviousDay=$(ls . | grep 'd[0-9]\+' | tr --delete d | sort --numeric-sort | tail --lines 1)
CurrentDay="$(expr "$PreviousDay" + 1)"

cp -r d1 d"$CurrentDay"

sed -i 's/d1/d'"$CurrentDay"'/g' "d$CurrentDay/Cargo.toml"
sed -i '/^\]$/i\ \ \ \ "'"d$CurrentDay"'",' Cargo.toml
