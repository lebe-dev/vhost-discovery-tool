# Site Discovery Flea

Утилита сбора ссылок из nginx и apache для мониторинга. Вывод результатов в формате Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/4.0/ru/manual/discovery/low_level_discovery).

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

## Настройка Zabbix агента

1.Копируем исполняемый файл `site-discovery-flea` в `/etc/zabbix`.

2.Обновляем права:

```
chown -R zabbix.zabbix /etc/zabbix
chmod +x /etc/zabbix/site-discovery
```

3.Создаем файл конфигурации `/etc/zabbix/zabbix-agent.d/site-discovery.conf` с содержимым:

```
UserParameter=site.discovery,/etc/zabbix/site-discovery-flea --include-custom-ports
UserParameter=vhost.index-page.available[*],/usr/bin/curl -s -i $1 | head -1 | cut -d " " -f 2 | grep '[200|302]' > /dev/null; echo $?;
```

4. Добавляем на Zabbix Server к хосту шаблон `VirtualHosts`.

## RoadMap

### 1.1.0

- Опция: Включать Endpoint'ы  
  Например, проксирование вида `proxy_pass ...` выдавать как отдельный URL 