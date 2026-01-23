FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install additional packages for Rust compilation and general development
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    udev \
    libudev-dev
