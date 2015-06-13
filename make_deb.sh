#!/bin/bash
set -e

# sudo apt-get install -y build-essential devscripts debhelper
#
# This will not work on any platform that isn't amd64

mkdir -m 0755 -p deb-build
cd deb-build

mkdir -m 0755 -p httptoircbridge_0.0.1-1
cd httptoircbridge_0.0.1-1

# Copy binary, strip debug symbols
mkdir -m 0755 -p usr/bin
strip -s -o usr/bin/http-to-irc-bridge ../../target/release/http-to-irc-bridge
chmod 755 usr/bin/http-to-irc-bridge

# Control file
mkdir -m 0755 -p DEBIAN
cp ../../control DEBIAN/control

# Add changelog and copyright
mkdir -m 0755 -p usr/share/doc/httptoircbridge/
chmod 0755 usr
chmod 0755 usr/share
chmod 0755 usr/share/doc
gzip -9 ../../changelog -c > usr/share/doc/httptoircbridge/changelog.Debian.gz
cp ../../copyright usr/share/doc/httptoircbridge/
chmod 644 usr/share/doc/httptoircbridge/*

# Build deb
cd ..
fakeroot dpkg-deb --build httptoircbridge_0.0.1-1