# MicroRoulette - Provably Fair Linera Roulette
# Uses pre-built base image for fast startup (no 2+ hour build!)
# Base image: mujeebmuzaffar/linera-base:0.15.8

FROM mujeebmuzaffar/linera-base:0.15.8 as builder

WORKDIR /app
COPY . .

# Build contracts
RUN cd contracts && cargo build --release --target wasm32-unknown-unknown

# Build frontend
RUN cd frontend && npm install && npm run build

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    openssl \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

COPY --from=builder /usr/local/cargo/bin/linera* /usr/local/bin/
COPY --from=builder /app /app

WORKDIR /app
RUN chmod +x init.sh

EXPOSE 8080 8081

CMD ["./init.sh"]
