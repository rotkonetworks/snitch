# snitch

Snitch is a streamlined monitoring tool that enhances the capabilities
of Prometheus Watchdog by integrating Pushover notifications. It offers
a unique functionality of triggering notifications through API calls for
users with valid API keys.


```
``snitch/
│
├── src/
│   ├── main.rs             # Entry point, setup of HTTP server, middleware
│   ├── settings.rs         # Configuration settings, struct definitions for config
│   ├── api/
│   │   ├── mod.rs          # API mod to hold all version modules
│   │   ├── v1/             # Version 1 of the API
│   │   │   ├── mod.rs      # Declare v1 API module and sub-modules
│   │   │   ├── handlers.rs # Request handlers specific to v1
│   │   │   ├── models.rs   # Data models and business logic for v1
│   │   │   └── routes.rs   # Route declarations for v1
│   │   └── v2/             # Future version 2 of the API
│   ├── services/
│   │   ├── mod.rs          # Services module for business logic layer
│   │   ├── alert_service.rs# Business logic for alerting features
│   │   ├── watchdog_service.rs# Business logic for alerting features
│   │   └── db_services.rs # Business logic for data handling
│   ├── db/
│   │   ├── mod.rs          # DB module for database interactions
│   │   └── conn.rs         # Database connection setup
│   │   └── redis.rs        # Stateless cache connection setup
│   ├── utils/
│   │   ├── mod.rs          # Utility functions and helpers
│   │   └── auth.rs         # Authentication utility functions
│   └── models/
│       ├── mod.rs          # Models for DB and domain entities
│       ├── alert_log.rs    # Endpoint specific models
│       └── app_state.rs    # AppState specific models
│
├── tests/                  # Integration and unit tests
│   ├── api_tests.rs
│   └── service_tests.rs
│
├── Cargo.toml              # Rust project manifest
├── Cargo.lock              # Automatically generated lock file
└── config.toml.sample      # Sample configuration file
```

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
   docker compose build
   docker compose up -d
   # or alternatively use deploy.sh script
   ```

## Integration with Prometheus

Refer to the `alertmanager.yml` in the repository for guidance on integrating
Snitch with Prometheus. This configuration assists in setting up alert rules
and establishing a connection with Snitch.

Default alertmanager.yml values and snitch configs match following Watchdog rule(2m / none):
```yaml
  rules:
  - alert: Watchdog
    expr: vector(1)
    for: 2m
    labels:
      severity: none
    annotations:
      summary: "Constant watchdog alert for monitoring the alerting pipeline."
      description: "This alert is always firing to ensure that the alerting pipeline is functional."
```

## Usage

- **/watchdog Endpoint:** This endpoint serves as a dead man's switch to confirm
the ongoing operation of your monitoring setup. It's crucial for ensuring that
your monitoring system is active and functioning correctly.
- **/alert Endpoint:** Ideal for situations where immediate, manual intervention
is needed. This endpoint allows authorized users to send urgent alerts.

## Contributions

We welcome contributions to Snitch! Please feel free to submit pull requests
or raise issues for any new features, bug fixes, or suggestions.

## License

Snitch is released under the [MIT License](LICENSE).
