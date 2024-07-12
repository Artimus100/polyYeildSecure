# Use a slim version of Rust for the base image
FROM rust:1.79-slim-buster

# Show backtraces for easier debugging
ENV RUST_BACKTRACE=1

# Install additional dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git \
    clang \
    libclang-dev \
    pkg-config \
    libssl-dev \
    cmake \
    file \
    curl \
    ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Configure Git to use CLI for fetching and disable SSL verification
RUN git config --global url."https://".insteadOf git:// && \
    git config --global http.sslVerify false

# Install Rustup and add the wasm32-unknown-unknown target
RUN curl -k https://sh.rustup.rs -sSf | sh -s -- -y && \
    /root/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain 1.79.0

# Set environment variable to use CLI for fetching Git repositories
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Copy the Substrate node template source code into the container
COPY . /usr/src/substrate-node-template
WORKDIR /usr/src/substrate-node-template

# Build the Substrate node template
RUN /root/.cargo/bin/cargo build --release

# Expose necessary ports
EXPOSE 9930 9333 9944 30333 30334

# Run the node in development mode
CMD ["./target/release/node-template", "--dev"]
# Use a slim version of Rust for the base image
FROM rust:1.79-slim-buster

# Show backtraces for easier debugging
ENV RUST_BACKTRACE=1

# Install additional dependencies
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
    git \
    clang \
    libclang-dev \
    pkg-config \
    libssl-dev \
    cmake \
    file \
    curl \
    ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Configure Git to use CLI for fetching and disable SSL verification
RUN git config --global url."https://".insteadOf git:// && \
    git config --global http.sslVerify false

# Install Rustup and add the wasm32-unknown-unknown target
RUN curl -k https://sh.rustup.rs -sSf | sh -s -- -y && \
    /root/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain 1.79.0

# Set environment variable to use CLI for fetching Git repositories
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

# Copy the Substrate node template source code into the container
COPY . /usr/src/substrate-node-template
WORKDIR /usr/src/substrate-node-template

# Build the Substrate node template
RUN /root/.cargo/bin/cargo build --release

# Expose necessary ports
EXPOSE 9930 9333 9944 30333 30334

# Run the node in development mode
CMD ["./target/release/node-template", "--dev"]
