#!/usr/bin/env sh

hub='docker.io/dominicparga'
image_name='devcontainer'
tag='20230514'

root_dirpath=$(dirname "$(readlink -f "${0}")")/..

docker build \
    --file "${root_dirpath}/.github/workflows/Dockerfile" \
    --tag "${hub}/${image_name}:${tag}" \
    --tag "${hub}/${image_name}:latest" \
    .

# docker push "${hub}/${image_name}:${tag}"
# docker push "${hub}/${image_name}:latest"