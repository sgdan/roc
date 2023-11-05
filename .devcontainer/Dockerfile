FROM ubuntu:23.10 as roc-dev
WORKDIR /working

# git required in dev container
# curl & xz-utils used for zig install
RUN apt-get -y update \
    && apt-get -y install \
    git curl xz-utils \
    cargo llvm libpolly-16-dev zlib1g-dev libzstd-dev build-essential

# Install Zig
ARG ZIG_VERSION=0.11.0
ARG ZIG_FOLDER=zig-linux-x86_64-$ZIG_VERSION
RUN curl https://ziglang.org/download/$ZIG_VERSION/$ZIG_FOLDER.tar.xz \
    | tar xJ -C /opt
ENV PATH /opt/$ZIG_FOLDER:$PATH

# Support lld (optional, see BUILDING_FROM_SOURCE.md)
RUN apt-get -y install lld-16
COPY <<EOF /root/.cargo/config.toml
[build]
rustflags = ["-C", "link-arg=-fuse-ld=lld", "-C", "target-cpu=native"]
EOF
ENV PATH /usr/lib/llvm-16/bin:$PATH

FROM roc-dev as roc-test
COPY crates ./crates
copy examples ./examples
COPY .cargo ./.cargo
COPY .llvmenv .
COPY *.toml .
COPY *.nix .
COPY Earthfile .
COPY flake.lock .
COPY Cargo.lock .
COPY version.txt .
# run "cargo build"
