#!/usr/bin/env bash
export LIBCLANG_PATH=/tmp/libclang_extract/usr/lib/x86_64-linux-gnu
export LD_LIBRARY_PATH=/tmp/libclang_extract/usr/lib/x86_64-linux-gnu:/tmp/alsa_dev/usr/lib/x86_64-linux-gnu
export PKG_CONFIG_PATH=/tmp/alsa_dev/usr/lib/x86_64-linux-gnu/pkgconfig
export CC=/usr/bin/gcc
exec /home/askal/cam_ratatui/target/release/cam_ratatui
