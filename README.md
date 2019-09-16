# Site Discovery Flea

Утилита для обнаружения доменных имен в конфигих nginx и apache. Вывод результатов в формате Zabbix Low Level Discovery.

## Пример вывода

```$json
{
    "data":[
        {
            "{#URL}":"https://somesite.ru"
        },
        {
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