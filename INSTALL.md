# How to install

1. Download [fresh release](https://github.com/tinyops-ru/zabbix-lld-ws/releases)

2. Copy `wszl` to `/etc/zabbix` on Zabbix server

3. Set permissions:
    ```bash
    chmod +x /etc/zabbix/wszl
    ```
4. Create config file `/etc/zabbix/wszl.yml`:
    ```bash
    cp wszl.yml-dist /etc/zabbix/wszl.yml
    ```

   Update permissions:
    ```shell script
    chmod 750 /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```

5. Add cron task (i.e. `/var/spool/cron/zabbix`):
    ```cronexp
    */30 * * * * /etc/zabbix/wszl --log-level=info -d /etc/zabbix gen
    ```   
   Every 30 minutes tool will generate required items.

## Related

- [Configuration example](EXAMPLE.md)