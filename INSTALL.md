# How to install

1. Copy `wszl` to `/etc/zabbix` on Zabbix server

2. Set permissions:
    ```bash
    chmod +x /etc/zabbix/wszl
    ```
3. Create config file `/etc/zabbix/wszl.yml`:
    ```bash
    cp wszl.yml-example /etc/zabbix/wszl.yml
    ```

   Update permissions:
    ```shell script
    chmod 750 /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```

4. Add cron task (i.e. `/var/spool/cron/zabbix`):
    ```
    */30 * * * * /etc/zabbix/wszl gen
    ```   
   Every 30 minutes tool will generate required items.

## Related

[Configuration example](EXAMPLE.md)