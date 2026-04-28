#!/usr/bin/env bash

# TODO: Undo allowing of missing docs
cargo clippy -- -D clippy::all -D clippy::pedantic -A clippy::missing-errors-doc
