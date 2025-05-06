#!/usr/bin/env bash
set -x
set -eo pipefail

RUNNING_REDIS_CONTAINER=$(podman ps --filter 'name=valkey' --format '{{.ID}}')
if [[ -n $RUNNING_REDIS_CONTAINER ]]; then
  echo >&2 "there is a redis container already running, kill it with"
  echo >&2 "    podman kill ${RUNNING_CONTAINER}"
  exit 1
fi

# Launch Valkey using Docker
podman run \
    -p "6379:6379" \
    -d \
    --name "valkey_$(date '+%s')" \
    valkey/valkey:8.1-alpine

>&2 echo "Valkey (Redis) is ready to go!"
