FROM rust:latest as build

RUN cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release

RUN rm ./target/release/deps/backend*
RUN rm src/*.rs
COPY . .

RUN cargo build --release

FROM debian:latest

ARG USERNAME=user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

RUN mkdir -p /home && chown $USERNAME:$USERNAME /home
RUN mkdir -p /home/logs && chown $USERNAME:$USERNAME /home/logs

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME

WORKDIR /home

EXPOSE 3000
COPY --from=build /backend/target/release/backend .

CMD ["./backend"]
