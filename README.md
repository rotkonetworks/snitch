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

## Prometheus
See alertmanager.yml for example configuration.
