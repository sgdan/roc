FROM ubuntu:23.10 as roc-dev
WORKDIR /working
RUN apt-get -y update \
    && apt-get -y install \
    
    # support git in dev container
    git \

    # needed to install Zig
    curl xz-utils \

    # support cargo build
    build-essential rustc cargo llvm clang libpolly-16-dev zlib1g-dev libzstd-dev

ARG ZIG_VERSION=0.11.0
ARG ZIG_FOLDER=zig-linux-x86_64-$ZIG_VERSION
RUN curl https://ziglang.org/download/$ZIG_VERSION/$ZIG_FOLDER.tar.xz \
    | tar xJ -C /opt
ENV PATH /opt/$ZIG_FOLDER:$PATH

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

FROM nixos/nix:2.18.1 as nix-dev
RUN nix-channel --update
WORKDIR /working
RUN echo "experimental-features = nix-command" >> /etc/nix/nix.conf
RUN echo "extra-experimental-features = flakes" >> /etc/nix/nix.conf
ENV LLVM_SYS_160_PREFIX=/usr/lib/llvm-16

FROM nix-dev as nix-test
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
# run "nix develop -c cargo build"
# downloads 9G into /nix/store
# fails with: No suitable version of LLVM was found system-wide or pointed to by LLVM_SYS_160_PREFIX.