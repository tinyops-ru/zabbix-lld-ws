# WSZL

Добавляет поддержку Web-сценариев для механизма [Zabbix Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery).

## С чего начать

### Установка

1. Установите утилиту [vhdt](https://github.com/lebe-dev/vhost-discovery-tool) для низкоуровневого обнаружения виртуальных хостов из nginx\apache.
2. Скопируйте исполняемый файл `wszl` в `/etc/zabbix` на Zabbix-сервере
3. Установите право на исполнение:
    ```bash
    chmod +x /etc/zabbix/wszl
    ```
4. Создайте файл конфигурации `/etc/zabbix/wszl.yml`:
    ```bash
    cp wszl.yml-example /etc/zabbix/wszl.yml
    ```
    Отредактируйте файл, укажите имя пользователя и пароль для доступа к Zabbix API.
   
    Обновите права:
    ```bash
    chmod o-rwx /etc/zabbix/wszl.yml
    chown zabbix: /etc/zabbix
    ```
    
5. Добавьте задачу в планировщик cron (например, в файл `/var/spool/cron/zabbix`):
    Каждые 30 минут утилита будет создавать Web-сценарии и триггеры для обнаруженных элементов.
    ```
    */30 * * * * /etc/zabbix/wszl gen
    ```   

### Использование

#### Создание Web-сценариев и триггеров

```
$ wszl gen
```

#### Конфигурация

Файл `wszl.yml`.

## Как работает утилита

1. Утилита ищет элементы по маске vhost.item через Zabbix API. Маску можно переопределить опцией.
2. Затем создает вэб-сценарии и триггеры
    - Параметры Web-сценария:
      - Заголовок вида: "Check index page 'XYZ'"
      - Ожидаемый код ответа: 200
    - Параметры триггера: 
      - Уровень приоритета - High (4), 
      - Заголовок вида: 'Site XYZ is unavailable'
      - Выражение: `web.test.fail`

## Версия Zabbix API

```yaml
zabbix:
  api:
    version: 6 # Поддерживаемые значения: 6, 5
```

## Решение проблем

Подробный лог `wszl.log`.

Вы можете менять уровни логирования с помощью опции `--log-level`.

## Спасибо за участие

- [cuchac](https://github.com/cuchac)