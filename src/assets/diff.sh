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
  .metadata.annotations["kubectl.kubernetes.io/last-applied-configuration"],
  .metadata.annotations["argocd.argoproj.io/tracking-id"]
  )' {} \;

diff "${DIFF_ARGS[@]}" "$@" | awk '!/^diff/ {if ($1 ~ /(---|\+\+\+)/) {print $1, $2} else {print $0}}'    
