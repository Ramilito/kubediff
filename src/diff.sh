#!/bin/bash
DIFF_ARGS=(
  "--minimal"
  "--width=120"
  "-u"
  "-N"
)
if [[ ! $ANSIBLE_MODE = YES ]]; then
  DIFF_ARGS+=("--color=always")
fi

find "$@"/* -type f  -print0 | xargs -0 -n 1 \
  yq e -i 'del(.metadata.managedFields, .metadata.annotations == with_entries(select(.key == "kubectl.kubernetes.io/last-applied-configuration")))'

diff "${DIFF_ARGS[@]}" "$@"   
