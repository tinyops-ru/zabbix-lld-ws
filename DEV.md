# Development

## Integration tests

**Requirements:**

- docker
- [vhdt](https://github.com/lebe-dev/vhost-discovery-tool)

Start fresh Zabbix Server:

```shell
rm -rf data
docker-compose up -d
```

Login to Zabbix with Admin http://localhost:3080 with creds: `Admin` / `zabbix`.

Create host `test`.

Use `vhdt` tool at least once it will create required host items.

Then run tests:

```shell
chmod +x run-integration-tests.sh
./run-integration-tests.sh
```