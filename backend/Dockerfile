# ----------------------
# 1) Build Stage
# ----------------------
# Pinning to 1.94-bookworm for stability
FROM rust:1.94-bookworm AS builder
WORKDIR /usr/src/app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Build actual application
COPY . .
# We use touch to ensure the compiler realizes the 'dummy' main.rs has changed
RUN touch src/main.rs && cargo build --release

# ----------------------
# 2) Runtime Stage
# ----------------------
# Using the matching slim bookworm image
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Security: Run as a non-root user
RUN useradd -ms /bin/bash appuser
USER appuser

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/backend ./backend
# COPY --from=builder /usr/src/app/migrations ./migrations

EXPOSE 8030
CMD ["./backend"]