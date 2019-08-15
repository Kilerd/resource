FROM rust:latest as rust
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --force --version 0.21.0 cargo-make
WORKDIR /app
COPY . /app
WORKDIR /app/client
RUN cargo make all_release

FROM clux/muslrust:stable as builder

WORKDIR /app

RUN USER=root cargo new resource
WORKDIR /app/resource

COPY --from=builder /app/server/Cargo.toml ./
COPY --from=builder /app/server/Cargo.lock ./

RUN echo 'fn main() { println!("Dummy") }' > ./src/main.rs

RUN cargo build --release

RUN rm -r target/x86_64-unknown-linux-musl/release/.fingerprint/resource-*

COPY --from=builder /app/server/src src/
COPY --from=builder /app/server/migrations migrations/
COPY --from=builder /app/server/templates templates/

RUN cargo build --release --frozen --bin resource


FROM alpine:latest

COPY --from=builder /app/resource/migrations /application/migrations
COPY --from=builder /app/resource/templates /application/templates
COPY --from=builder /app/resource/target/x86_64-unknown-linux-musl/release/resource /application/resource

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/resource
ENV TELEGRAM_BOT_SECRET_KEY TELEGRAM_BOT_SECRET_KEY
ENV TELEGRAM_WHITE_LIST TELEGRAM_WHITE_LIST

WORKDIR /application

ENV RUST_LOG resource=INFO

CMD ["./resource"]