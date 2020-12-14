# Build Stage
FROM rust:1.47 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/nhql

# Cache dependencies
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Dummy file for fast build
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build

# Build
COPY . .
RUN cargo install --path .

# Runtime
FROM gcr.io/distroless/cc

COPY --from=build /usr/local/cargo/bin/nhql /usr/local/bin/nhql
# COPY --from=build /usr/src/nhql/static static

CMD ["nhql"]
