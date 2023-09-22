# How to build

## Linux

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