FROM rust:1.86-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY cloud-stack-report/ cloud-stack-report/
COPY cloud-health-full-report/ cloud-health-full-report/
COPY cloud-mail-full-report/ cloud-mail-full-report/
COPY cloud-url-health-report/ cloud-url-health-report/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates openssh-client curl dnsutils iputils-ping netcat-openbsd \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/cloud-stack-report /usr/local/bin/
COPY --from=builder /build/target/release/cloud-health-full /usr/local/bin/
COPY --from=builder /build/target/release/cloud-mail-full /usr/local/bin/
COPY --from=builder /build/target/release/cloud-url-health /usr/local/bin/
LABEL org.opencontainers.image.source="https://github.com/diegonmarcos/cloud-data"
WORKDIR /workspace
