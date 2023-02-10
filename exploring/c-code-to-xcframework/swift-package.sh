#!/bin/bash

set -e

./xcframework.sh

rm -rf swift-cmd/.build
rm -rf build/MyMath.package
mkdir build/MyMath.package

cp -r build/libmymath.xcframework build/MyMath.package/libmymath.xcframework
cp Package.swift build/MyMath.package/Package.swift

cd build/MyMath.package
swift package resolve

cd ../../swift-cmd
swift run