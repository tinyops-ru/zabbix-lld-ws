#!/bin/bash

RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu

cp target/release/zabbix-lld-ws wszl

strip wszl
eu-elfcompress wszl
upx -9 --lzma wszl

version=`cat Cargo.toml | grep version | head -1 | cut -d "\"" -f 2`

zip -9 -r wszl-${version}-linux-amd64-static.zip wszl README.md README.RU.md urls.txt-example wszl.yml-dist
