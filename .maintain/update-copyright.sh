#!/usr/bin/env bash

SINGLE_DATES=$(grep -lr "// Copyright (C) [0-9]* MIT open source license.")
YEAR=$(date +%Y)

for file in $SINGLE_DATES; do
  FILE_YEAR=$(cat $file | sed -n "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\) MIT open source license.|\1|p")
  if [ $YEAR -ne $FILE_YEAR ]; then
    sed -i -e "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\) MIT open source license.|// Copyright (C) \1-$YEAR MIT open source license.|g" $file
  fi
done

grep -lr "// Copyright (C) [0-9]*-[0-9]* MIT open source license." |
  xargs sed -i -e "s|// Copyright (C) \([[:digit:]][[:digit:]][[:digit:]][[:digit:]]\)-[[:digit:]][[:digit:]][[:digit:]][[:digit:]] MIT open source license.|// Copyright (C) \1-$YEAR MIT open source license.|g"
