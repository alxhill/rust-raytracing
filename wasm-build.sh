#!/usr/local/bin/env bash

wasm-pack build rt-wasm

cd rt-wasm/html/
npm ci
