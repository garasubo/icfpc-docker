FROM rust:slim-stretch
WORKDIR /source

COPY . .
WORKDIR /source/app
ENTRYPOINT [ "cargo" "run" "--release" "--offline" ]
