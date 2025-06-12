FROM rust:1.86.0 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/task-web-actix /usr/local/bin/app

CMD ["cargo", "watch", "-x", "run"]