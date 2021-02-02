#!/bin/bash

cargo build --release

rm deploy/esm-admin
cp -rf target/release/esm-admin deploy
echo "replaced new file for deploy/esm-admin"

cd deploy
echo "starting new esm-admin"
./esm-admin