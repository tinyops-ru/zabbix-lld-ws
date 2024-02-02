# WSZL

[Русская версия](README.RU.md)

Adds Web Scenarios support for 
[Zabbix Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## Getting started

### Installation

See [INSTALL.md](INSTALL.md).

### Usage

#### Generate items and triggers

```shell
wszl --log-level=info -d /etc/zabbix gen
```

#### Use file as a source for urls

By default wszl gets urls from Zabbix, but you can provide urls from file.

```shell
wszl --log-level=info -d /etc/zabbix gen --source=file --file=urls.txt
```

Check `urls.txt-example` as example.

#### Configuration

File `wszl.yml`.

**Options:**

- `--item-key-starts-with` - specify item search mask

## How it works

1. WSZL gets items from Zabbix API by mask
2. Creates web scenarios and triggers
    - Web scenario params: title - "Check index page 'XYZ'", expected response code - 200
    - Trigger params: severity - High (4), title - 'Site XYZ is unavailable', expression `web.test.fail`  

## Zabbix API version

[Zabbix API v6 only](https://www.zabbix.com/documentation/6.0/en/manual/api)

## Troubleshooting

Check `wszl.log` file for details.

You can switch logging levels with `--log-level` option. Example:

```shell
wszl --log-level=debug gen
```

## Contributors

- [cuchac](https://github.com/cuchac)