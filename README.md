# WSZL

[Русская версия](README.RU.md)

Adds Web Scenarios support for 
[Zabbix Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## Getting started

### Installation

See [INSTALL.md](INSTALL.md).

### Usage

#### Generate items and triggers

```
$ wszl --log-level=info -d /etc/zabbix gen
```

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

```yaml
zabbix:
  api:
    version: 6 # Supported values: 6, 5
```

## Troubleshooting

Check `wszl.log` file for details.

You can switch logging levels with `--log-level` option. Example:

```shell
wszl --log-level=debug gen
```

## Contributors

- [cuchac](https://github.com/cuchac)