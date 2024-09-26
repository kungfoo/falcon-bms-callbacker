#!/bin/bash
set -xe

version=`git describe --tags --always --dirty`
echo "Packaging ${version}..."

cargo build --release
rm -rf target/package/
mkdir -p target/package/
cp target/release/falcon_bms_callbacker.exe target/package/falcon_bms_callbacker-${version}.exe
cp config-release.toml target/package/config.toml

pushd target/package/
zip -9 falcon_bms_callbacker-${version}.zip *
popd

echo "Done."