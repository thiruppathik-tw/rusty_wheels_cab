#!/bin/bash

#Add target
rustup target add aarch64-unknown-linux-gnu

#Install gcc for the target
brew tap messense/macos-cross-toolchains
brew install aarch64-unknown-linux-gnu