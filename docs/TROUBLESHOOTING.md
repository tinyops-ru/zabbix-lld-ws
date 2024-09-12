# Troubleshooting

Check `wszl.log` file for details.

You can switch logging levels with `--log-level` option. Example:

```shell
wszl --log-level=debug gen
```

### Web-scenarios weren't created

First of all, check that your zabbix user has appropriate permissions.

Auth:

```shell
curl --location --request GET 'https://zabbix.company.com/api_jsonrpc.php' \
--header 'Content-Type: application/json' \
--data '{
    "jsonrpc": "2.0",
    "method": "user.login",
    "params": {
        "username": "nginx-vhost-discovery",
        "password": "YOUR-PASSWORD"
    },
    "id": 1
}'
```

Response:

```json
{
    "jsonrpc": "2.0",
    "result": "3ea03e491d238a5ea2820fab2d5a0ce6",
    "id": 1
}
```

Get items:

```shell
curl --location --request GET 'https://zabbix.company.com/api_jsonrpc.php' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer 3ea03e491d238a5ea2820fab2d5a0ce6' \
--data '{
    "jsonrpc": "2.0",
    "method": "item.get",
    "params": {
        "output": "extend",        
        "with_triggers": false,
        "sortfield": "name",
        "search": {
            "key_": "nginx.vhost.item"
        }
    },
    "id": 1
}'
```

Response must contain items.