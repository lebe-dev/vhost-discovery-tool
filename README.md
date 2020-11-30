# Site Discovery Flea

Утилита сбора ссылок из nginx и apache для мониторинга. Вывод результатов в формате Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/4.0/ru/manual/discovery/low_level_discovery).

## Настройка Zabbix агента

1.Копируем исполняемый файл `site-discovery-flea` в `/var/lib/zabbix`.

2.Обновляем права:

```
chown -R zabbix.zabbix /var/lib/zabbix
chmod +x /var/lib/zabbix/site-discovery-flea
```

3.Создаем файл конфигурации `/etc/zabbix/zabbix-agent.d/site-discovery.conf` с содержимым:

```
UserParameter=site.discovery,/var/lib/zabbix/site-discovery-flea
UserParameter=vhost.index-page.available[*],/usr/bin/curl -s -i $1 | head -1 | cut -d " " -f 2 | grep '[200|302]' > /dev/null; echo $?;
```

4. Добавляем на Zabbix Server к хосту шаблон `VirtualHosts`.

## Опции

### Показывать в результате хосты с нестандартными портами

Опция: `--include-custom-ports`

В результатах будут также хосты вида http://somehost.ru:3823

Под стандартными портами понимаются: 80-й и 443-й 

## Пример вывода

```$json
{
    "data":[
        {
            "{#NAME}":"somesite.ru",
            "{#URL}":"https://somesite.ru"
        },
        {
            "{#NAME}":"15.128.42.21:2231",
            "{#URL}":"http://15.128.42.21:2231"
        }
    ]
}
```

## Решение проблем

### Ошибки типа Permission denied в логах Zabbix

#### Нет доступа к каталогу /etc/nginx

**Решение:**

```
setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
```

или

```
setfacl -Rm u:zabbix:rx /etc/nginx/sites-enabled
```

#### Неправильные права на log файл
Возможно вы запускали агента от пользователя `root`, утилита создала файл `/var/log/zabbix/site-discovery-flea.log` и не может
туда писать т. к. не имеет прав.

**Решение:** 

```
chown -R zabbix: /var/log/zabbix/site-discovery-flea.log
```

## RoadMap

### 1.1.0

- Опция: Включать Endpoint'ы  
  Например, проксирование вида `proxy_pass ...` выдавать как отдельный URL 