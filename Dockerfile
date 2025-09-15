# Builder Stage
FROM rust:1.86-bookworm AS builder

WORKDIR /app

COPY . .
RUN cargo build --release --bins --package relayer-base --package ton

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /usr/local/bin

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/* ./
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]