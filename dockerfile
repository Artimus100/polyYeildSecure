FROM debian:bookworm

# Install necessary packages including musl
RUN apt-get update && apt-get install -y \
    gcc \
    build-essential \
    libssl-dev \
    pkg-config \
    g++ \
    libc6-dev \
    musl-tools \
    clang \
    musl \
    musl-dev

# Set up environment for cross-compilation
ENV CC_x86_64_unknown_linux_gnu=x86_64-linux-musl-gcc

# Copy your project files
COPY . .

# Build the project
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Copy the binary to a new image or run it in the same container
COPY target/x86_64-unknown-linux-gnu/release/polyYieldSecuree /polyYieldSecuree

# Set the entry point
ENTRYPOINT ["/polyYieldSecuree"]
