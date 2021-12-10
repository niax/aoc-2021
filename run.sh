#!/bin/bash

set -xe

IMAGE=niax/aoc2021

docker image inspect "${IMAGE}" >/dev/null 2>&1 || docker build .docker -t "${IMAGE}"

exec docker run --rm -i -v "${PWD}:/code" -v "${PWD}/.cargo:/usr/local/cargo/registry" "${IMAGE}" "${@}"
