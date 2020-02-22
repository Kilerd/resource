FROM clux/muslrust:stable as builder

WORKDIR /app

RUN USER=root cargo new resource
WORKDIR /app/resource
COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release
# RUN rm -r target/x86_64-unknown-linux-musl/release/.fingerprint/resource-*

COPY src src/
COPY migrations migrations/
COPY templates templates/
COPY page page/
COPY data data/
RUN cargo build --release --bin resource

FROM alpine:latest

COPY --from=builder /app/resource/migrations /application/migrations
COPY --from=builder /app/resource/templates /application/templates
COPY --from=builder /app/resource/data /application/data
COPY --from=builder /app/resource/page /application/page
COPY --from=builder /app/resource/target/x86_64-unknown-linux-musl/release/resource /application/resource

EXPOSE 8000

ENV DATABASE_URL postgres://root@postgres/resource

WORKDIR /application

ENV RUST_LOG resource=INFO

CMD ["./resource"]