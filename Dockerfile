FROM ubuntu:20.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y build-essential git curl cmake clang libclang-dev \
    llvm llvm-dev python3-dev python3-numpy libgtk2.0-dev pkg-config libavcodec-dev \
    libavformat-dev libswscale-dev libtbb2 libtbb-dev libcanberra-gtk-module \
    libcanberra-gtk3-module libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev libopencv-dev \
    pkg-config libssl-dev

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain 1.71.0 -y && \
    ln -s $HOME/.cargo/bin/* /usr/bin/

RUN rustup default nightly-unknown-linux-gnu

COPY . /home/docsearcher
WORKDIR /home/docsearcher

RUN cargo build

ENTRYPOINT [ "./target/debug/searcher" ]
