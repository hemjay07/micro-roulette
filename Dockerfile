# MicroRoulette - Provably Fair Linera Roulette
# Uses pre-built base image for fast startup (no 2+ hour build!)
# Base image: mujeebdimeji/linera-base:0.15.8

FROM mujeebdimeji/linera-base:0.15.8 as builder

# Build arguments for Vite
ARG VITE_APP_ID
ARG VITE_CHAIN_ID
ARG VITE_LINERA_FAUCET_URL=https://faucet.testnet-conway.linera.net

# Set as environment variables for build
ENV VITE_APP_ID=$VITE_APP_ID
ENV VITE_CHAIN_ID=$VITE_CHAIN_ID
ENV VITE_LINERA_FAUCET_URL=$VITE_LINERA_FAUCET_URL

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
    && apt-get install -y nodejs \
    && npm install -g serve

COPY --from=builder /usr/local/cargo/bin/linera* /usr/local/bin/
COPY --from=builder /app /app

WORKDIR /app
COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x docker-entrypoint.sh init.sh

EXPOSE 8080 8082

CMD ["./docker-entrypoint.sh"]
