FROM ubuntu:20.04

WORKDIR /app

RUN apt-get update && apt-get install -y \
    build-essential \
    curl

# install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

# make cargo command visiable everywhere
ENV PATH="/root/.cargo/bin:${PATH}"

# move cargo cache location to /app, which will be mapped to host
ENV CARGO_HOME="/app"
