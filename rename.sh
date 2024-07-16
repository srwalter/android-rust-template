#!/bin/bash

set -e

cratename="$1"
shift
javaname="$1"
shift

cratename_underscores="${cratename//-/_}"
echo $cratename_underscores
short_javaname="$(echo $javaname | cut -d. -f3)"
echo $short_javaname

sed -ie "s/android-rust-template/$cratename/g" Cargo.toml
sed -ie "s/android_rust_template/$cratename_underscores/g" AndroidManifest.xml src/bin/desktop.rs

sed -ie "s/com.example.rustapp/$javaname/g" AndroidManifest.xml Makefile
sed -ie "s/rustapp/$short_javaname/g" Makefile src/platform/android/mod.rs
