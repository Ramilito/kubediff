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


find "$@" -type f -exec yq e -i 'del(
  .metadata.managedFields,
  .metadata.ownerReferences,
  .metadata.generation,
  .metadata.creationTimestamp,
  .webhooks,
  .data,
  .spec.caBundle,
  .metadata.annotations == with_entries(select(.key == "kubectl.kubernetes.io/last-applied-configuration"))
  )' {} \;

diff "${DIFF_ARGS[@]}" "$@" | awk '!/^diff/ {if ($1 ~ /(---|\+\+\+)/) {print $1, $2} else {print $0}}'    
