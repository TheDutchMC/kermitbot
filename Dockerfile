
FROM rust:1.56.1-slim-buster as API_BUILDER
RUN apt update && apt install -y \
    musl-tools \
    pkgconf
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/
COPY ./src /usr/src/src/
COPY ./migrations /usr/src/migrations
COPY ./Cargo.toml /usr/src

ENV RUSTFLAGS='-C link-arg=-s'
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM node:16-bullseye-slim as FRONTEND_BUILDER
RUN apt update && apt install -y \
    make

WORKDIR /usr/src
COPY ./frontend /usr/src/

RUN make dist

# Runtime image
FROM alpine:latest
RUN apk add --no-cache ca-certificates
COPY --from=API_BUILDER /usr/src/target/x86_64-unknown-linux-musl/release/kermitbot /usr/local/bin/kermitbot
COPY --from=FRONTEND_BUILDER /usr/src/dist /usr/local/bin/frontend_dist/

RUN chmod a+rx /usr/local/bin/*
RUN adduser kermitbot -s /bin/false -D -H
USER kermitbot

ENV DOCKERIZED=TRUE
ENV RUST_LOG=kermitbot=info

EXPOSE 8080
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/kermitbot" ]