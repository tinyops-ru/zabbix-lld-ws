# Specify url data source

Application collects hosts and urls from zabbix items such as `vhost.item[http://github.com]`.

You able to provide zabbix hosts and urls from text file:

```shell
./wszl gen -s file -f urls.txt
```

File should contain records in format: `zabbix-host-name|url`.

Example:

```
github|https://github.com
intel.com|https://intel.com
```

Rows with any other format will be ignored.