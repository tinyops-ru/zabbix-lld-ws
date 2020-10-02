# Web scenarios for Zabbix Low Level Discovery

Add support Web Scenarios to 
Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) feature.

### How it works

1. Tool get items from Zabbix API and filter by regular expression
2. Check each item:  
    1. Check web scenario item availability, creates if doesn't exist. Creates trigger.
    2. Check http-agent item availability, creates item and trigger if doesn't exist. 
    Add trigger dependency to web scenario.
