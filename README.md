# WSZL

Add Web Scenarios support to [Zabbix Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery).

## Why?

Zabbix team provides very fragile http-item implementation and don't want to support 
for Web-scenarios in Low Level Discovery feature. So, I've implemented this tool to fix that.

## Installation

See [INSTALL.md](INSTALL.md).

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

Check `urls.txt-example` as example.

## How it works

1. WSZL gets items from Zabbix API by mask or file
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