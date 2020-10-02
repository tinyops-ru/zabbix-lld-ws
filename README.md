# Web scenarios for Zabbix Low Level Discovery

Add support Web Scenarios to 
Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

## How it works

1. Tool get items from Zabbix API and filter by regular expression
2. Check each item:  
    1. Check web scenario item availability, creates if doesn't exist. Creates trigger.
    2. Check http-agent item availability, creates item and trigger if doesn't exist. 
    Add trigger dependency to web scenario.

## Getting started

### Installation

1. Copy `wszl` to `/etc/zabbix` on Zabbix server
2. Set permissions:
    ```shell script
    chmod +x /etc/zabbix/wszl
    ```
3. Create credentials file `/etc/zabbix/.wszl-credentials`:
    ```shell script
    username=CHANGE-ME
    password=CHANGE-ME
    ```   
    Set username and password for access to Zabbix API.
   
    Update permissions:
    ```shell script
    chmod o-rwx /etc/zabbix/.wszl-credentials
    chown zabbix: /etc/zabbix
    ```
    
4. Add cron task:
    ```
    */30 * * * * /etc/zabbix/wszl gen
    ```   
    Every 30 minutes tool will generate proper items.
