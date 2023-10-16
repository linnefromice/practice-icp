FROM --platform=linux/amd64 ubuntu:22.04

ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo
ENV RUST_VERSION=1.73.0
ENV DFX_VERSION=0.14.3

# Install a basic environment needed for our build tools
RUN apt -yq update && \
    apt -yqq install --no-install-recommends curl ca-certificates \
        build-essential pkg-config libssl-dev llvm-dev liblmdb-dev clang cmake rsync \
        wget

# Install Rust and Cargo
ENV PATH=/opt/cargo/bin:${PATH}
RUN curl --fail https://sh.rustup.rs -sSf \
        | sh -s -- -y --default-toolchain ${RUST_VERSION}-x86_64-unknown-linux-gnu --no-modify-path
RUN rustup default ${RUST_VERSION}-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

# Install dfx
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
