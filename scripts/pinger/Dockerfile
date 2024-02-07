FROM rust:1.67 as builder
WORKDIR /usr/src/pinger
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/pinger /usr/local/bin/pinger
ENTRYPOINT ["pinger"]
