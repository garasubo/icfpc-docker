FROM rust:slim-stretch
WORKDIR /source

COPY . .
WORKDIR /source/app
RUN cargo run --release --offline
