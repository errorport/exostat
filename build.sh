#!/bin/bash

RUSTFLAGS="-Ctarget-cpu=skylake"

echo -e "\e[32;1mBuilding...\e[0"
cargo build --release
echo -e "\e[32;1mStripping binaries...\e[0"
strip target/release/exostat

