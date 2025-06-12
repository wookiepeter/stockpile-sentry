# Dockerfile

# ---- Stage 1: Build Stage ----
# Use the official Rust image as a builder.
# TODO: potentially replace with latest or something similar
FROM rust:1.74-slim-bullseye as builder

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Install dependencies and build the release binary.
RUN cargo build --release

# ---- Stage 2: Final Stage ----
# Use a minimal, secure base image for the final container.
FROM debian:bullseye-slim

# Set the working directory
WORKDIR /bin

# Copy only the compiled binary from the builder stage.
COPY --from=builder /app/target/release/stockpile-sentry ./stockpile-sentry

# IMPORTANT: Replace 'your_app_name' with the actual name of your binary,
# which is usually the name of your crate defined in Cargo.toml.

# Expose the port your application listens on.
EXPOSE 8000

# Command to run the application when the container starts.
CMD ["./stockpile-sentry"]