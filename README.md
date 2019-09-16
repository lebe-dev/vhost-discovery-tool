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