FROM rust:1.67 as builder
WORKDIR /usr/src/fizzbuzz
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/fizzbuzz /usr/local/bin/fizzbuzz
ENTRYPOINT ["fizzbuzz"]
