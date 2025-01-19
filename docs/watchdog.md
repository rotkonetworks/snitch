## Watchdog API

#### Overview
The `watchdog` receiver is an integral part of our alert management system,
specifically designed to monitor the health and responsiveness of our services.
It acts as a fail-safe mechanism that ensures our system is alive and functioning
as expected.

It's meant to use with prometheus alertmanager, and it's configured to send
heartbeats to the `watchdog` receiver when the alert is in `firing` state.
The receiver interacts with a watchdog service hosted at
`https://domain/api/v1/watchdog`, sending alert notification if there
is no response within required interval.

To setup prometheus look into these repositories:

- [member-prometheus](https://github.com/ibp-network/member-prometheus)
- [prometheus-ansible](https://github.com/ibp-network/prometheus-ansible)


### alertmanager.yaml
```yaml
global:
  # Global configurations...

route:
  receiver: 'default-receiver'
  group_by: ['alertname', 'severity']
  group_wait: 30s
  group_interval: 1m
  repeat_interval: 10m

  routes:
    - match:
        alertname: 'Watchdog'
      receiver: 'watchdog'
      group_wait: 15s
      group_interval: 30s
      repeat_interval: 1m

    - match:
        severity: 'none'
      receiver: 'watchdog'
      repeat_interval: 1m

    - match:
        severity: 'critical'
      receiver: 'pushover'

    - match:
        severity: 'warning'
      receiver: 'pushover'

receivers:
  - name: 'default-receiver'

  - name: 'pushover'
    pushover_configs:
      - token: 'app_token_here'
        user_key: 'user_key_here'

  - name: 'watchdog'
    webhook_configs:
      - url: 'https://rpc.rotko.net/watchdog'
        send_resolved: true
        http_config:
          authorization:
            type: Bearer
            credentials: '4ad3ef73-2f55-4817-9fbe-056f95a54f7a'
```
