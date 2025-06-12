FROM rust:1.86.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/tempserver /usr/local/bin/app

# Make sure the binary is executable
RUN chmod +x /usr/local/bin/app

# Expose the port your app runs on
EXPOSE 5000

CMD ["/usr/local/bin/app"]