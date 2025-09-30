#!/bin/bash
set -xe

version=`git describe --tags --always --dirty`
echo "Packaging ${version}..."

cargo build --release --target x86_64-pc-windows-gnu
rm -rf target/package/
mkdir -p target/package/win64/
upx --best target/x86_64-pc-windows-gnu/release/falcon_bms_callbacker.exe
cp target/x86_64-pc-windows-gnu/release/falcon_bms_callbacker.exe target/package/win64/falcon_bms_callbacker-win64-${version}.exe
cp config-release.toml target/package/win64/config.toml

pushd target/package/win64/
zip -9 falcon_bms_callbacker-win64-${version}.zip *
popd

#cargo build --target=i686-pc-windows-msvc --release

#mkdir -p target/package/win32/
#cp target/i686-pc-windows-msvc/release/falcon_bms_callbacker.exe target/package/win32/falcon_bms_callbacker-win32-${version}.exe
#cp config-release.toml target/package/win32/config.toml

#pushd target/package/win32/
#gzip -9 falcon_bms_callbacker-win32-${version}.zip *
#popd


echo "Done."
