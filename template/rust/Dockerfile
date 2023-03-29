FROM ghcr.io/openfaas/of-watchdog:0.9.11 as watchdog
FROM rust:1.56-alpine as build

WORKDIR /home/rust

# Copy all the sources
COPY main ./main
COPY function ./function

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cd main && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.17.2 as ship

COPY --from=watchdog /fwatchdog /usr/bin/fwatchdog
RUN chmod +x /usr/bin/fwatchdog

ARG ADDITIONAL_PKG=""
# Install packages and add non-root user
RUN apk --no-cache add curl ca-certificates ${ADDITIONAL_PKG} \
    && addgroup -S app && adduser -S -g app app

# Split instructions so that buildkit can run & cache
# the previous command ahead of time.
RUN mkdir -p /home/app \
    && chown app /home/app

WORKDIR /home/app

COPY --from=build --chown=app:app /home/rust/main/target/x86_64-unknown-linux-musl/release/main .

USER app

# Set up watchdog for HTTP mode
ENV fprocess="./main"
ENV mode="http"
ENV upstream_url="http://127.0.0.1:3000"

EXPOSE 8080
HEALTHCHECK --interval=3s CMD [ -e /tmp/.lock ] || exit 1

CMD ["fwatchdog"]
