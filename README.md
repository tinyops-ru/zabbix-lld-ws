# WSZL

> Under development

Add support Web Scenarios to 
Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## How it works

1. WSZL gets items from Zabbix API and filters by mask
2. Check each item:  
    1. Check web scenario item availability, creates if doesn't exist. Creates trigger.

## Getting started

### Installation

1. Copy `wszl` to `/etc/zabbix` on Zabbix server
2. Set permissions:
    ```shell script
    chmod +x /etc/zabbix/wszl
    ```
3. Create config file `/etc/zabbix/wszl.yml`:
    ```shell script
    cp wszl.yml-example /etc/zabbix/wszl.yml
    ```
   
    Update permissions:
    ```shell script
    chmod o-rwx /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```
    
4. Add cron task:
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

### Troubleshooting

Check `wszl.log` file for details.

You can switch logging levels with `--log-level` option.

### Roadmap

- Remove generated items
