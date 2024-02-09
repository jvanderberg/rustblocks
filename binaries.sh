#!/bin/sh
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin
cp ./target/x86_64-apple-darwin/release/rustblocks ./bin/MacOS
cp ./target/x86_64-pc-windows-gnu/release/rustblocks.exe ./bin/Windows
