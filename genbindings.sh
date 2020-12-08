#!/bin/bash

set -ex
(
	cd txkit-c-api
	rustup run nightly cbindgen -o ../include/txkit.h -c ../cbindgen.toml
)
