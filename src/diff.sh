#!/bin/bash
DIFF_ARGS=(
  "--minimal"
  "--width=120"
  "-I" "^  generation:"
  "-I" "^    deprecated.daemonset.template.generation:"
  "-u"
  "-N"
)
if [[ ! $ANSIBLE_MODE = YES ]]; then
  DIFF_ARGS+=("--color=always")
fi

diff "${DIFF_ARGS[@]}" "$@" | awk '!/^diff/ {if ($1 ~ /(---|\+\+\+)/) {print $1, $2} else {print $0}}'    
