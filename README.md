# Site Discovery Flea

Утилита для обнаружения доменных имен в конфигих nginx и apache. Вывод результатов в формате [Low Level Discovery](https://www.zabbix.com/documentation/4.0/ru/manual/discovery/low_level_discovery).

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

1.Копируем исполняемый файл `site-discovery` в `/etc/zabbix`.

2.Обновляем права:

```
chown -R zabbix.zabbix /etc/zabbix
chmod +x /etc/zabbix/site-discovery
```

3.Редактируем конфигурацию агента `/etc/zabbix/zabbix-agent.d/site-discovery`, добавляем:

```
UserParameter=site.discovery,/etc/zabbix/site-discovery
```

## RoadMap

### 1.1.0

- Опция: Включать Endpoint'ы  
  Например, проксирование вида `proxy_pass ...` выдавать как отдельный URL 