#!/bin/sh
cargo  build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./web/out --target web ./target/wasm32-unknown-unknown/release/led_simulator.wasm