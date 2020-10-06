# WSZL

> Under development

Add support Web Scenarios to 
Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## Getting started

### Installation

1. Setup [site discovery flea](https://github.com/lebe-dev/site-discovery-flea)  
   It provides low level discovery for virtual hosts (nginx or apache).
2. Copy `wszl` to `/etc/zabbix` on Zabbix server
3. Set permissions:
    ```shell script
    chmod +x /etc/zabbix/wszl
    ```
4. Create config file `/etc/zabbix/wszl.yml`:
    ```shell script
    cp wszl.yml-example /etc/zabbix/wszl.yml
    ```
   
    Update permissions:
    ```shell script
    chmod o-rwx /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```
    
5. Add cron task:
    ```
    */30 * * * * /etc/zabbix/wszl gen
    ```   
    Every 30 minutes tool will generate proper items.

### Usage

#### Generate items and triggers

```
$ wszl gen
```

#### Configuration

File `wszl.yml`.

## How it works

1. WSZL gets items from Zabbix API by mask
2. Creates missing web scenarios and triggers
    - Web scenario params: title - "Check index page 'XYZ'", expected response code - 200
    - Trigger params: severity - High (4), title - 'Site XYZ is unavailable', expression `web.test.fail`  

## Troubleshooting

Check `wszl.log` file for details.

You can switch logging levels with `--log-level` option.

## Roadmap

- Control web scenario parameters with command options (title, return code, etc.)
- Control trigger parameters with command options (title, severity, description, etc.)
- Remove generated items
