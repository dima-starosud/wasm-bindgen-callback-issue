#!/usr/bin/env bash

set -ex

pushd rust-impl
wasm-pack build --target bundler
popd

pushd ts-usage
yarn add ../rust-impl/pkg
npx tsc --outDir dist
node dist/index.js
popd
