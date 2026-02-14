# Multi-stage build for Bitcoin Digital Labor Derivative API
# Optimized for Render.com free tier

# Build stage - using nightly for edition2024 support
FROM rustlang/rust:nightly-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY tests ./tests
COPY examples ./examples

# Build the application in release mode with API feature
RUN cargo build --release --bin api-server --features api

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 -s /bin/bash bdld

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/api-server /app/api-server

# Change ownership to non-root user
RUN chown -R bdld:bdld /app

# Switch to non-root user
USER bdld

# Expose port 8080 (Render.com default)
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV BDLD_PORT=8080
ENV BDLD_ENV=production

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/bin/sh", "-c", "exec 3<>/dev/tcp/127.0.0.1/8080 && echo -e 'GET /health HTTP/1.1\\r\\nHost: localhost\\r\\nConnection: close\\r\\n\\r\\n' >&3 && cat <&3 | grep -q 'healthy'"]

# Run the application
CMD ["/app/api-server"]
