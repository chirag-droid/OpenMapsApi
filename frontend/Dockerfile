FROM rust:latest as build

ARG TILE_SERVER="/tile"

RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall -y trunk

RUN cargo new --bin frontend
WORKDIR /frontend

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --target wasm32-unknown-unknown --release
RUN rm ./target/wasm32-unknown-unknown/release/deps/frontend*

RUN rm src/*.rs

COPY src/ src/
COPY scss/ scss/
COPY index.html index.html
COPY Trunk.toml Trunk.toml

ENV TILE_SERVER=${TILE_SERVER}

RUN trunk build

FROM nginx:alpine AS RUNTIME
EXPOSE 3000

COPY --from=build /frontend/dist /frontend
COPY .conf/nginx.conf /etc/nginx/nginx.conf
