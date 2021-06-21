# Virtual Host Discovery Tool

Утилита сбора ссылок (URL) из nginx и apache для мониторинга. Вывод результатов в формате Zabbix 
[Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery).

Для версии Zabbix ниже 4.2 используйте опцию `--use-data-property` (см.раздел Опции).

## С чего начать

1. Копируем исполняемый файл `vhdt` в `/usr/bin`.
2. Создаем файл конфигурации:
  ```
  cp vhost-discovery.conf /etc/zabbix/zabbix-agent.d/
  ```
3. Даём необходимые права:
  ```
  chmod +x /usr/bin/vhdt
  chown -R zabbix: /var/log/zabbix
  setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
  setfacl -Rm u:zabbix:rx /etc/nginx/sites-enabled
  ```
4. Добавляем на Zabbix Server к хосту шаблон `Virtual Hosts` (прилагается в виде файла `vhost-discovery-template.xml`).
Шаблон идет с дистрибутивом утилиты.
5. (Опционально) Устанавливаем утилиту [wszl tool](https://github.com/tinyops-ru/zabbix-lld-ws), которая создаёт Web-сценарии на базе обнаруженных vhosts.

## Как работает утилита

Утилита идет в конфиги Apache и Nginx и извлекает оттуда доменные имена и порты. На базе этих данных она формирует
данные для Low Level Discovery:

- Домен
- Ссылка

### Nginx: server_name с несколькими доменами

Например, `server_name tinyops.ru www.tinyops.ru`.

Утилита соберёт только `tinyops.ru` и проигнорирует остальные значения. 

Создайте тикет с обоснованием, почему вам нужны все домены кроме первого.

### HTTP

Для доменов с протоколом HTTP добавляет постфикс `_http`. Например, для сайта `http://somesite.ru` будет такая структура:
```json
{
  "{#NAME}":"somesite.ru_http",
  "{#URL}":"http://somesite.ru"
}
```

### Обработка конфигов nginx

Если в `server` не указано значение для `server_name`, то данный виртуальный хост игнорируется. 

## Опции

### Указать рабочую директорию

Опция: `--work-dir` или `-d`

Значение по умолчанию: `/etc/zabbix`

### Указать путь к конфигурациям nginx

Опция: `--nginx-vhosts-path` или `-n`

Значение по умолчанию: `/etc/nginx/conf.d`

### Указать путь к конфигурациям apache

Опция: `--apache-vhosts-path` или `-a`

Значение по умолчанию: `/etc/httpd/conf.d`

### Показывать в результате хосты с нестандартными портами

Опция: `--include-custom-ports`

В результатах будут также хосты вида `http://somehost.ru:3823`.

Под стандартными портами понимаются: 80-й и 443-й 

### Фильтрация по имени домена

Опция: `--ignore-by-masks` or `-i`

Пример:

```bash
vhdt -i test,rfid
```

Исключит из результатов vhosts с доменами, которые содержат одну или несколько масок: `test` or `rfid`.

### Поддержка версий до 4.2

До версии Zabbix 4.2 использовался JSON формат такого вида:

```json
{
  "data": []
}
``` 

В поздних версиях отказались от свойства `data`.

Чтобы включить поддержку старого формата используйте опцию `--use-data-property`

## Пример вывода

```json
[
    {
        "{#NAME}":"somesite.ru",
        "{#URL}":"https://somesite.ru"
    },
    {
        "{#NAME}":"15.128.42.21:2231",
        "{#URL}":"http://15.128.42.21:2231"
    }
]
```

## Решение проблем

Утилита пишет свой лог в файл `/var/log/zabbix/vhdt.log`.

Запуск утилиты от пользователя `zabbix`:

```shell script
sudo -u zabbix /usr/bin/vhdt
```

### Нет доступа к каталогу /etc/nginx

Решение:

```bash
setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
```

или

```bash
setfacl -Rm u:zabbix:rx /etc/nginx/sites-enabled
```

### Уровни логирования

Можно управлять уровнем логирования через флаг `--log-level`.

Поддерживаемые значения: `debug`, `error`, `warn`, `trace`, `info`, `off`

### Как отключить логирование?

```shell script
--log-level=off
```

## Спасибо за поддержку

Спасибо за поддержку проекта, тестирование и обратную связь:

- [ttsrg](https://github.com/ttsrg)
- [BTLzdravtech](https://github.com/BTLzdravtech)
