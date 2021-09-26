FROM rust:slim

RUN apt update
RUN apt install -y pkg-config libssl-dev
RUN adduser --gecos '' --disabled-login --home /app app
USER app
WORKDIR /app
COPY --chown=app Cargo.toml Cargo.toml
COPY --chown=app Cargo.lock Cargo.lock
COPY --chown=app coffeenote-api coffeenote-api
COPY --chown=app coffeenote-domain coffeenote-domain
RUN mkdir target .cargo
RUN --mount=type=cache,target=/app/.cargo,uid=1000,gid=1000 cargo install sqlx-cli --no-default-features --features postgres
RUN --mount=type=cache,target=/app/.cargo,uid=1000,gid=1000 \
    --mount=type=cache,target=/app/target,uid=1000,gid=1000 \
    cargo check