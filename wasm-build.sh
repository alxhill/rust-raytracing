#!/usr/bin/env bash

set -e -o pipefail

wasm-pack build rt-wasm --debug

cd rt-wasm/html/
npm ci
