FROM rust:1.81 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/bible_api /usr/local/bin/bible_api
COPY bibles ./bibles
CMD ["bible_api"]
