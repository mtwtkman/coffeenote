# syntax = docker/dockerfile:1.2
FROM rust:slim

RUN apt update
RUN apt install -y pkg-config libssl-dev curl
RUN adduser --gecos '' --disabled-login --home /app app
RUN mkdir grpcurl && \
    cd grpcurl && \
    curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.8.5/grpcurl_1.8.5_linux_x86_64.tar.gz -o grpcurl.tar.gz && \
    tar -zxvf grpcurl.tar.gz && \
    mv grpcurl /usr/local/bin && \
    cd ~ && \
    rm -rf grpcurl
USER app
WORKDIR /app
COPY --chown=app Cargo.toml Cargo.toml
COPY --chown=app Cargo.lock Cargo.lock
COPY --chown=app coffeenote-api coffeenote-api
COPY --chown=app coffeenote-core coffeenote-core
RUN mkdir target .cargo
RUN rustup component add rustfmt
RUN --mount=type=cache,target=/app/.cargo,uid=1000,gid=1000 cargo install sqlx-cli --no-default-features --features postgres
RUN --mount=type=cache,target=/app/.cargo,uid=1000,gid=1000 \
    --mount=type=cache,target=/app/target,uid=1000,gid=1000 \
    cargo check