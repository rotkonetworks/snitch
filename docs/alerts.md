## Alerts API

Alerts API is to send alerts to endpoint to alert directly in alerting
application. Here are 3 acceptable ways to use the endpoint.

### 1. Send an alert with a predefined alert message:

```bash
curl -X POST -H "Authorization: Bearer api_key" http://127.0.0.1:7000/api/v1/alert
```

### 2. Send an alert with a message parameter via a query string:

```bash
curl -X POST -H "Authorization: Bearer api_key" http://127.0.0.1:7000/api/v1/alert?message=moilolita
```

### 3. Send a JSON payload with a message:

```bash
curl -X POST \
  -H "Authorization: Bearer api_key" \
  -H "Content-Type: application/json" \
  -d '{"message": "lol"}' \
  http://127.0.0.1:7000/api/v1/alert
```
