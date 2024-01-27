# snitch

Snitch is a streamlined monitoring tool that enhances the capabilities
of Prometheus Watchdog by integrating Pushover notifications. It offers
a unique functionality of triggering notifications through API calls for
users with valid API keys.

## Features
- Seamless integration with Prometheus Watchdog.
- Pushover notifications for efficient alert dissemination.
- API endpoints for specialized alerting purposes:
  - `/watchdog`: A dead man's switch endpoint for monitoring system vitality.
  - `/alert`: An endpoint for triggering immediate, high-priority alerts.

## Getting Started

Here's how to set up Snitch for your monitoring needs.

### Prerequisites
- Docker
- Git (to clone the repository)

### Installation Steps

1. **Clone the Repository**
   ```bash
   git clone https://github.com/rotkonetworks/snitch.git
   cd snitch
   ```

2. **Configuration**
   Copy and personalize the sample configuration file.
   ```bash
   cp config.toml.sample config.toml
   # Update config.toml with your Pushover API keys
   ```

3. **Deployment with Docker**
   Deploy Snitch using Docker Compose.
   ```bash
   docker compose up -d
   ```

## Integration with Prometheus

Refer to the `alertmanager.yml` in the repository for guidance on integrating
Snitch with Prometheus. This configuration assists in setting up alert rules
and establishing a connection with Snitch.

## Usage

- **/watchdog Endpoint:** This endpoint serves as a dead man's switch to confirm
the ongoing operation of your monitoring setup. It's crucial for ensuring that
your monitoring system is active and functioning correctly.
- **/alert Endpoint:** Ideal for situations where immediate, manual intervention
is needed. This endpoint allows authorized users to send urgent alerts.

## SSL Integration with HAProxy

For secure communication, it's recommended to configure HAProxy/nginx with SSL
to handle HTTPS traffic for the `/watchdog` and `/alert` endpoints. This ensures
encrypted data transmission, enhancing the security of your monitoring system.
More details in the example provided in our repository for basic SSL setup.

## Contributions

We welcome contributions to Snitch! Please feel free to submit pull requests
or raise issues for any new features, bug fixes, or suggestions.

## License

Snitch is released under the [MIT License](LICENSE).
