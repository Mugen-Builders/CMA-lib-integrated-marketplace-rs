# syntax=docker.io/docker/dockerfile:1.4
FROM ubuntu:22.04 AS builder

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.90.0

ARG DEBIAN_FRONTEND=noninteractive
RUN <<EOF
set -e
apt update
apt install -y --no-install-recommends \
    build-essential=12.9ubuntu3 \
    ca-certificates=20240203~22.04.1 \
    g++-riscv64-linux-gnu=4:11.2.0--1ubuntu1 \
    libclang-dev \
    wget=1.21.2-2ubuntu1
EOF

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c' ;; \
    armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='3b8daab6cc3135f2cd4b12919559e6adaee73a2fbefb830fadf0405c20231d61' ;; \
    arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='e3853c5a252fca15252d07cb23a1bdd9377a8c6f3efa01531109281ae47f841c' ;; \
    i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='a5db2c4b29d23e9b318b955dd0337d6b52e93933608469085c924e0d05b1df1f' ;; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.28.2/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup target add riscv64gc-unknown-linux-gnu

# Verify the RISC-V cross-linker is available
RUN which riscv64-linux-gnu-g++ || (echo "riscv64-linux-gnu-g++ not found" && exit 1)

WORKDIR /opt/cartesi/dapp

# Copy Cargo files first for better layer caching
COPY Cargo.toml Cargo.lock ./
COPY lib/cma-rust-parser/Cargo.toml lib/cma-rust-parser/Cargo.toml

# Copy Cargo configuration for cross-compilation linker settings
COPY .cargo ./.cargo

# Copy build dependencies needed for bindgen
COPY lib/cma-rust-parser/build.rs ./lib/cma-rust-parser/build.rs
COPY lib/cma-rust-parser/wrapper.h ./lib/cma-rust-parser/wrapper.h
COPY lib/cma-rust-parser/lib ./lib/cma-rust-parser/lib

# Copy source code
COPY src ./src
COPY lib/cma-rust-parser/src ./lib/cma-rust-parser/src

# Build with explicit target for cross-compilation
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/opt/cartesi/dapp/target \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    cargo build --release --target riscv64gc-unknown-linux-gnu --jobs $(nproc) && \
    cp /opt/cartesi/dapp/target/riscv64gc-unknown-linux-gnu/release/dapp /opt/cartesi/dapp/dapp

FROM --platform=linux/riscv64 ubuntu:22.04

ARG MACHINE_EMULATOR_TOOLS_VERSION=0.14.1
ADD https://github.com/cartesi/machine-emulator-tools/releases/download/v${MACHINE_EMULATOR_TOOLS_VERSION}/machine-emulator-tools-v${MACHINE_EMULATOR_TOOLS_VERSION}.deb /
RUN dpkg -i /machine-emulator-tools-v${MACHINE_EMULATOR_TOOLS_VERSION}.deb \
    && rm /machine-emulator-tools-v${MACHINE_EMULATOR_TOOLS_VERSION}.deb

LABEL io.cartesi.rollups.sdk_version=0.9.0
LABEL io.cartesi.rollups.ram_size=128Mi

ARG DEBIAN_FRONTEND=noninteractive
RUN <<EOF
set -e
apt-get update
apt-get install -y --no-install-recommends \
    busybox-static=1:1.30.1-7ubuntu3
rm -rf /var/lib/apt/lists/* /var/log/* /var/cache/*
useradd --create-home --user-group dapp
EOF

ENV PATH="/opt/cartesi/bin:/opt/cartesi/dapp:${PATH}"

WORKDIR /opt/cartesi/dapp
COPY --from=builder /opt/cartesi/dapp/dapp .

ENV ROLLUP_HTTP_SERVER_URL="http://127.0.0.1:5004"

ENTRYPOINT ["rollup-init"]
CMD ["dapp"]
