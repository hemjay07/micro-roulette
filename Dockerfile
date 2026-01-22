FROM rust:1.86.0 as builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    cmake \
    clang \
    openssl \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust WASM target
RUN rustup target add wasm32-unknown-unknown

# Install Linera tools
RUN cargo install linera-service@0.15.8 --locked
RUN cargo install linera-storage-service@0.15.8 --locked

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

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
