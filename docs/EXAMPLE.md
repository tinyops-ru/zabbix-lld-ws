# Example of configuration

Example describes how to monitor nginx host with dynamic virtual hosts.

## Option 1: wszl + vhdt

1. Install [vhdt](https://github.com/lebe-dev/vhost-discovery-tool) for virtual hosts low level discovery (nginx or apache). It will collect all virtual hosts from nginx/apache and create zabbix items.

2. Install [WSZL](INSTALL.md)

## Option 2: Use file with urls

1. Prepare `urls.txt` based on example file `urls.txt-example`

2. Install [WSZL](INSTALL.md)