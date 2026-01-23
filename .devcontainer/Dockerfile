FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Install Node.js, MongoDB and other dependencies
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Install Node.js 20
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    # Install MongoDB 6.0
    && wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | apt-key add - \
    && echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/debian bookworm/mongodb-org/6.0 main" | tee /etc/apt/sources.list.d/mongodb-org-6.0.list \
    && apt-get update \
    && apt-get install -y mongodb-org \
    # Development tools
    && apt-get -y install --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    udev \
    libudev-dev \
    # Create mongo data dir
    && mkdir -p /data/db \
    && chown -R vscode:vscode /data/db

# Start MongoDB in the background
RUN echo "mongod --fork --logpath /var/log/mongodb.log --dbpath /data/db" >> /home/vscode/.bashrc
