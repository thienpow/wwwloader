#!/bin/bash
git pull
cargo build --release

rm ../esm-datastore/deploy/admin/esm-admin-warp
cp -rf target/release/esm-admin-warp ../esm-datastore/deploy/admin/
echo "replaced new file for deploy/admin"
