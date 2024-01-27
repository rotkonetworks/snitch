# Stage 1: Build the application
FROM rust:alpine as builder
WORKDIR /app
COPY . .
# Install build dependencies including OpenSSL
RUN apk update && \
    apk add --no-cache musl-dev gcc openssl-dev openssl-libs-static
# Compile the application
RUN cargo build --release

# Stage 2: Setup the runtime environment
FROM alpine:latest
# Add runtime dependencies for OpenSSL
RUN apk add --no-cache libgcc openssl
WORKDIR /usr/local/bin
# Copy the built binary
COPY --from=builder /app/target/release/snitch snitch
# Copy the config file
COPY config.toml .
CMD ["snitch"]
