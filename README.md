# Virtual Host Discovery Tool

Collect URLs from nginx/apache configs and output them in Zabbix [Low-Level Discovery](https://www.zabbix.com/documentation/current/manual/discovery/low_level_discovery) format.

Use `--use-data-property` option for Zabbix < 4.2 (see details in Options section).

## Quick start

1. Copy `vhdt` to `/usr/bin`.
2. Copy zabbix agent config `vhost-discovery.conf` to `/etc/zabbix/zabbix-agent.d/vhost-discovery.conf`
3. Import template file `vhost-discovery-template.xml` in Zabbix Server Admin Panel.
4. Update permissions:
  ```shell
  chmod +x /usr/bin/vhdt
  chown -R zabbix: /var/log/zabbix
  setfacl -Rm u:zabbix:rx /etc/nginx/conf.d
  setfacl -Rm u:zabbix:rx /etc/nginx/sites-enabled
  ``` 

5. Attach `Virtual Hosts` template to target host.
6. (Optional) Use [wszl tool](https://github.com/tinyops-ru/zabbix-lld-ws). It creates web-scenarios+triggers based on vhost items.

## How it works

Tool looking for nginx\apache configuration files then creates data structures for Low Level Discovery:

- domain
- url

### Limitations

#### 1. Nginx: Multiple values in server_name

Example: `server_name toys.com www.toys.com`

Domain `toys.com` will be collected.

#### 2. Redirect limitations

If your vhost has row:

```
return 301 http...
```

Not inside `location` directive, it will be excluded from results.

### HTTP
Add `_http` postfix for domain with http protocol. For example: `http://somesite.ru` will be:  

```json
{
  "{#NAME}":"somesite.ru_http",
  "{#URL}":"http://somesite.ru"
}
```

### Processing nginx configs

Tool ignores hosts which don't have `server_name` property. 

## Options

### Working directory

Option: `--work-dir` or `-d`

Default value: `/etc/zabbix`

### Recursive mode

Enable recursive scan for sub-directories.

Option: `-r`

Default value: `false`

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

### Filter vhosts by domain masks

Option: `--ignore-by-masks` or `-i`

Example:

```bash
vhdt -i "^test,rfid$,demo"
```

Will ignore vhosts with domain names starts with `test` or ends with `rfid` or contain `demo`.

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

### Couldn't access to /etc/nginx directory

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
- [bbx-github](https://github.com/bbx-github)
