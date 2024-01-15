#!/bin/bash

WSZL_ZABBIX_API_URL=http://localhost:3080/api_jsonrpc.php
WSZL_ZABBIX_API_USER=Admin
WSZL_ZABBIX_API_PASSWORD=zabbix
WSZL_EXAMPLE_HOST_ID=CHANGE-ME # Example: 10678
WSZL_EXAMPLE_HOST_NAME=test
WSZL_EXAMPLE_TRIGGER_NAME="Site 'https://company.com' is unavailable"
WSZL_EXAMPLE_WEBSCENARIO_NAME="Check index page '"

cargo test