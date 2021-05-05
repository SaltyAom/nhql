# * --- Build Stage ---
FROM rust:1.51-alpine AS builder
WORKDIR /usr/src/
RUN apk add --no-cache musl-dev ca-certificates cmake musl-utils libressl-dev

# Setup tools for building
RUN rustup target add x86_64-unknown-linux-musl

# ? Create dummy project for package installation caching
RUN USER=root cargo new nhql
WORKDIR /usr/src/nhql

COPY Cargo.toml Cargo.lock ./

RUN RUSTFLAGS='-C target-cpu=native' cargo build --release

# Build project
COPY src ./src
# COPY static ./static

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo install --target x86_64-unknown-linux-musl --path .

# * --- Compression Stage ---
FROM alpine:3.13 AS compressor
WORKDIR /usr/app

RUN apk add --no-cache upx binutils

COPY --from=builder /usr/local/cargo/bin/nhql .

RUN strip nhql -o app-striped
RUN upx app-striped --best --lzma -o app

# * --- Running Stage ---
FROM scratch

COPY --from=compressor /usr/app/app .
# COPY static ./static

EXPOSE 8080

CMD ["./app"]