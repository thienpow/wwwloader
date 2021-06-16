#!/bin/sh


CARGO_PATH="$HOME/.cargo/bin"

cd ~/wwwloader/

echo "building wwwloader"
$CARGO_PATH/cargo build --release

[ ! -d ".build" ] && mkdir .build
cp target/release/wwwloader ./.build/wwwloader

uptime