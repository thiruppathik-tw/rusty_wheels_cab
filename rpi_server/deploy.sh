#!/bin/bash

# Build for the target
cargo clean
cargo build --target=aarch64-unknown-linux-gnu

# Copy binary to Raspberry Pi. Replace the Ip with actual Ip.
scp -r target/aarch64-unknown-linux-gnu/debug/rpi_sample pi@"$1":/home/pi/test/