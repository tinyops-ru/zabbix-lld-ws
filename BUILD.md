# How to build

## Linux (debian / ubuntu)

```shell
apt install libssl-dev -y
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```

## Linux (docker)

1. Install [cross](https://github.com/cross-rs/cross)

2. Build:

```shell
cross build --release
```

Result will be here `target/[architecture]/zabbix-lld-ws`.

## Windows

```shell
cargo build --release
```