#!/usr/bin/env bash

set -ex

pushd rust-impl
wasm-pack build --target bundler
popd

pushd ts-usage
yarn add ../rust-impl/pkg
yarn ts-node src/index.ts
popd
