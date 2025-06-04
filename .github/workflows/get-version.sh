#!/usr/bin/env bash

CRATE=${1:?Specify crate name}
cargo metadata --format-version 1 --no-deps | jq --raw-output  '.packages | map(select(.name == "'${CRATE}'")) | .[0].version'
