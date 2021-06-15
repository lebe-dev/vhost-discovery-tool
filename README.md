# Virtual Host Discovery Tool

[Русская версия](README.RU.md)

Gather urls from nginx\apache configs then outputs in Zabbix 
[Low Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) format.

Use `--use-data-property` option for Zabbix < 4.2 (see details in Options section).

## Getting started

1. Copy `vhdt` to `/usr/bin`.
2. Update permissions:  
    ```shell script
    chmod +x /usr/bin/vhdt
    ```
   
3. Copy zabbix agent config `files/vhost-discovery.conf` to `/etc/zabbix/zabbix-agent.d/vhost-discovery.conf`

4. Import `files/vhost-discovery-template.xml` to Zabbix Server.

5. Update permissions:

```
chown -R zabbix: /var/log/zabbix
setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
``` 

6. Add `Virtual Hosts` template to target host.

7. Setup [wszl tool](https://github.com/tinyops-ru/zabbix-lld-ws). It creates web-scenarios+triggers based on vhost items.

## How it works

Tool looking for nginx\apache configuration files then creates data structures for Low Level Discovery:

- domain
- url

Add `_http` postfix for domain with http protocol. For example: `http://somesite.ru` will be:  

```json
{
  "{#NAME}":"somesite.ru_http",
  "{#URL}":"http://somesite.ru"
}
```

### Processing for nginx configs

Tool ignores hosts which don't have `server_name` property. 

## Options

### Working directory

Option: `--work-dir` or `-d`

Default value: `/etc/zabbix`

### Nginx configs root

Option: `--nginx-vhosts-path` or `-n`

Default value: `/etc/nginx/conf.d`

### Apache configs root

Option: `--apache-vhosts-path` or `-a`

Default value: `/etc/httpd/conf.d`

### Show results with custom ports

Standard ports: 80, 443

Option: `--include-custom-ports`

Example: `http://somehost.ru:3823`. 

### Support Zabbix < 4.2

Zabbix 4.2 has JSON format:

```json
{
  "data": []
}
``` 

Later versions don't support `data` property. Use `--use-data-property` option for that. 

## Output example

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

## Troubleshooting

Log: `/var/log/zabbix/vhdt.log`.

### Unable to access /etc/nginx directory

Fix:

```bash
setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
```

or

```bash
setfacl -Rm u:zabbix:rx /etc/nginx/sites-enabled
```

### Logging levels

Use `--log-level` option if you want to switch logging level.

Supported levels: `debug`, `error`, `warn`, `trace`, `info`, `off`

### How to disable logging

```
--log-level=off
```

## Thanks for support

Thanks for project support, testing and feedback:

- [ttsrg](https://github.com/ttsrg)
- [BTLzdravtech](https://github.com/BTLzdravtech)
