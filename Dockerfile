# ---- Build Stage ----
# Use the official Rust image based on Debian "Bullseye"
FROM rust:trixie AS build

# Install build dependencies required for native libraries (like OpenSSL)
# - build-essential: provides C/C++ compilers and tools
# - libssl-dev: provides development headers for OpenSSL
# - pkg-config: helps find compiler/linker flags for libraries
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application in release mode
# This creates statically linked binaries where possible
RUN cargo build --release

# ---- Final Stage ----
# Use a minimal Debian "Bullseye" slim image for the final container
FROM debian:trixie

# Install runtime dependencies for the compiled binary
# - openssl: provides the runtime libraries (libssl, libcrypto)
# - ca-certificates: needed for making secure HTTPS requests
RUN apt-get update && apt-get install -y --no-install-recommends \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Arguments for creating a non-root user, with default values
ARG RUNNER_GROUP_ID=1000
ARG RUNNER_USER_ID=1000

# Create a group and user to run the application securely
RUN groupadd --gid ${RUNNER_GROUP_ID} appgroup && \
    useradd --uid ${RUNNER_USER_ID} --gid ${RUNNER_GROUP_ID} --shell /bin/bash --create-home appuser

# Set environment variables for the application
ENV APP=lupt
ENV RUST_LOG="actix_web=info"

# Switch to the non-root user
USER appuser
WORKDIR /home/appuser/app

# Copy the compiled binaries from the build stage
# --chown ensures the files are owned by the non-root user
COPY --from=build --chown=appuser:appgroup /app/target/release/server server
COPY --from=build --chown=appuser:appgroup /app/target/release/cli cli

# Set the default command to run the server
CMD ["./server"]
