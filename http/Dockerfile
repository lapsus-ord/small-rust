FROM rust:1-slim-bookworm as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

WORKDIR /app/http

RUN cargo init
COPY ./http/Cargo.* .
COPY ./core /app/core
RUN cargo fetch

COPY ./http/src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /app/http/target/x86_64-unknown-linux-musl/release/small-rust /

ENV APP_HOST=0.0.0.0
ENV APP_PORT=80
EXPOSE 80

ENV RUST_LOG=small_rust=trace
CMD ["/small-rust"]
