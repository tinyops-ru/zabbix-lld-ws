# WSZL

Add Web Scenarios support to [Zabbix Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## Why?

Zabbix Low Level Discovery doesn't support web scenarios. Let's fix that :)

## Installation

See [INSTALL.md](docs/INSTALL.md).

## Usage

### Generate items and triggers

Based on data from Zabbix:

```shell
wszl -d /etc/zabbix gen
```

Or use file as a source for urls:

```shell
wszl -d /etc/zabbix gen --source=file --file=urls.txt
```

Check [urls.txt-example](urls.txt-example) as an example.

## How it works

1. WSZL gets items from Zabbix API by mask (`--item-key-starts-with`) or list of urls from file (`--file`).
2. Creates web scenarios and triggers.

## Zabbix API version

Tested with [v6](https://www.zabbix.com/documentation/6.0/en/manual/api).

It might work with v5.

## Troubleshooting

See [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)

## Contributors

- [cuchac](https://github.com/cuchac)