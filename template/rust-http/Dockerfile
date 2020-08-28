FROM openfaas/of-watchdog:0.7.2 as watchdog

FROM rust:1.37-alpine as builder

WORKDIR /home/rust

# Copy all the sources
COPY function ./function
COPY main ./main

RUN cd main && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.11 as runner

# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates \
    && addgroup -S app && adduser -S -g app app

ENV USER=app

# Copy of-watchdog binary
COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

# Copy main binary
COPY --from=builder /home/rust/main/target/x86_64-unknown-linux-musl/release/main /usr/bin/main
RUN chmod +x /usr/bin/main

# Set up watchdog for HTTP mode
ENV fprocess="main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
