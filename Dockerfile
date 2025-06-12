FROM rust:1.86.0 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim

# Install required dependencies including curl for health checks
RUN apt-get update && \
    apt-get install -y \
    libssl-dev \
    ca-certificates \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/task-web-actix /usr/local/bin/app

# Make sure the binary is executable
RUN chmod +x /usr/local/bin/app

# Expose the port your app runs on
EXPOSE 4000

CMD ["/usr/local/bin/app"]