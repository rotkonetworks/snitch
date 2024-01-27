# snitch
A simple monitoring tool to enable the usage of prometheus Watchdog with
pushover notifications. Includes also option to receive notifications via
api calls from api-key holders.

## Usage
```sh
git clone https://github.com/rotkonetworks/snitch.git
cd snitch
cp config.toml.sample -> config.toml
# Add your api keys to config.toml
docker compose up -d
```

## Prometheus Watchdog
Setup prometheus Watchdog to send alerts to https://rpc.rotko.net:7000/watchdog
```yaml
global:

route:
  routes:
    - match:
        alertname: Watchdog
      receiver: 'watchdog_receiver'

receivers:
- name: 'watchdog_receiver'
  webhook_configs:
  - url: 'https://rpc.rotko.net:7000/watchdog'
    http_config:
      custom_headers:
        X-Api-Key: 'your_api_key_here'
```
