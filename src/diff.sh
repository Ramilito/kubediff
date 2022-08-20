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

cat "$@"/* \
  | yq e 'del(.metadata.managedFields)' \
  | yq e 'del(.metadata.annotations == with_entries(select(.key == "kubectl.kubernetes.io/last-applied-configuration")))'

