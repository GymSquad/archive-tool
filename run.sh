#!/bin/sh

pywb_collections_path=$1

echo "pywb_collections_path=${pywb_collections_path}"

docker run --rm -it \
    --add-host host.docker.internal:host-gateway \
    -e DATABASE_URL="postgresql://app:app@localhost:5432/db" \
    -v ${pywb_collections_path}:/data \
    alan910127/archive-tool:latest