FROM openfaas/of-watchdog:0.7.2 as watchdog

# We are going to build static Rust binary
# Reference: https://github.com/emk/rust-musl-builder
FROM ekidd/rust-musl-builder:latest AS builder

WORKDIR /home/rust

# Cargo requires $USER to be set
ENV USER=rust

# Cache dependencies

# Create empty projects
RUN cargo new --lib function && \
    cargo new --bin main

# Copy Cargo files
COPY function/Cargo.toml ./function/Cargo.toml
COPY --chown=rust:rust function/Cargo.lock ./function/Cargo.lock
COPY main/Cargo.toml ./main/Cargo.toml
COPY --chown=rust:rust main/Cargo.lock ./main/Cargo.lock

# Download and build deps
RUN cd function && cargo build
RUN cd main && cargo build

# Build the binary

# Copy all the sources
COPY --chown=rust:rust function ./function
COPY --chown=rust:rust main ./main

# And build in release mode
RUN cd main && cargo build --release --target=x86_64-unknown-linux-musl

# Prepare runtime image
FROM alpine:latest

WORKDIR /home/app

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates \
    && addgroup -S app && adduser -S -g app app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Copy the binary from builder
COPY --from=builder --chown=app:app \
    /home/rust/main/target/x86_64-unknown-linux-musl/release/main \
    /home/app/main

# Switch to non-root user
USER app

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
