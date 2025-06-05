FROM rust:latest

# Install wasm-pack, nodejs, and other build tools
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    nodejs \
    npm \
    && npm install -g serve \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Create working directory
WORKDIR /snake
