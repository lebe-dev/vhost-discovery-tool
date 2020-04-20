# Site Discovery Flea

Утилита сбора ссылок из nginx и apache для мониторинга. Вывод результатов в формате Zabbix [Low Level Discovery](https://www.zabbix.com/documentation/4.0/ru/manual/discovery/low_level_discovery).

Для версии Zabbix ниже 4.2 используйте опцию `--use-data-property` (см.раздел Опции).

## Настройка Zabbix агента

1.Копируем исполняемый файл `site-discovery-flea` в `/usr/bin`.

2.Обновляем права:

```
chmod +x /usr/bin/site-discovery-flea
```

3.Создаем файл конфигурации `/etc/zabbix/zabbix-agent.d/site-discovery.conf` с содержимым:

```
UserParameter=site.discovery,/usr/bin/site-discovery-flea
UserParameter=vhost.index-page.available[*],/usr/bin/curl -s -L -i $1 | grep "200 Ok" > /dev/null; echo $?
```

4. Добавляем на Zabbix Server к хосту шаблон `VirtualHosts`.

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

В результатах будут также хосты вида http://somehost.ru:3823

Под стандартными портами понимаются: 80-й и 443-й 

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

Утилита пишет свой лог в файл `/etc/zabbix/site-discovery-flea.log`.

## RoadMap

### 1.4.0

- Опция: Включать Endpoint'ы  
  Например, проксирование вида `proxy_pass ...` выдавать как отдельный URL 
  
### 1.3.0
  
  - Возможность управлять уровнем логирования

## Спасибо за поддержку

Спасибо за поддержку проекта, тестирование и обратную связь:

- [ttsrg](https://github.com/ttsrg)
