FROM rust:1.88-slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY reports-common/ reports-common/
COPY cloud-stack-report/ cloud-stack-report/
COPY cloud-health-full-report/ cloud-health-full-report/
COPY cloud-mail-full-report/ cloud-mail-full-report/
COPY cloud-url-health-report/ cloud-url-health-report/
COPY cloud-sec-network-report/ cloud-sec-network-report/
COPY cloud-sec-data-report/ cloud-sec-data-report/
COPY cloud-health-daily-mail/ cloud-health-daily-mail/
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates openssh-client curl dnsutils iputils-ping netcat-openbsd \
    openssl yara rsync \
    && rm -rf /var/lib/apt/lists/*

# Fetch community YARA rules at build time
RUN mkdir -p /usr/local/share/yara-rules/community && \
    curl -sL https://urlhaus.abuse.ch/downloads/urlhaus.rules \
      -o /usr/local/share/yara-rules/community/urlhaus.yar || true

# Copy binaries
COPY --from=builder /build/target/release/health-reporter /usr/local/bin/health-reporter
COPY --from=builder /build/target/release/cloud-health-full /usr/local/bin/
COPY --from=builder /build/target/release/cloud-mail-full /usr/local/bin/
COPY --from=builder /build/target/release/cloud-url-health /usr/local/bin/
COPY --from=builder /build/target/release/cloud-sec-network /usr/local/bin/
COPY --from=builder /build/target/release/cloud-sec-data /usr/local/bin/
COPY --from=builder /build/target/release/cloud-health-daily-mail /usr/local/bin/

# Bundle YARA rules (custom + infra)
COPY cloud-sec-data-report/yara-rules/ /usr/local/share/yara-rules/

# Entrypoint — same pattern as cloud-builder-x
COPY entrypoint.sh /opt/reports/entrypoint.sh
RUN chmod +x /opt/reports/entrypoint.sh

LABEL org.opencontainers.image.source="https://github.com/diegonmarcos/cloud-data"
LABEL org.opencontainers.image.description="cloud-data-reports — health report runner (SSH, WG, SOPS setup)"

WORKDIR /root/git/cloud-data/reports
ENTRYPOINT ["/opt/reports/entrypoint.sh"]
CMD ["all"]
