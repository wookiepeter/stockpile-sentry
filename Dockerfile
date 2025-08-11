# Dockerfile using cargo-chef for faster builds during development
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Stage 1 -> Build dependencies (if necessary)
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2 -> Build container 
# For now stick to debug -> later both commands should go to release
FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json
COPY . .
RUN cargo build --bin stockpile-sentry

# Stage 3 -> Run the image
FROM debian:trixie-slim
WORKDIR /app
# Copy only the compiled binary from the builder stage.
COPY --from=builder /app/target/debug/stockpile-sentry ./stockpile-sentry
# Expose the port for traefik
EXPOSE 8000
# Run
CMD ["./stockpile-sentry"]