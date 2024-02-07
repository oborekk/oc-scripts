FROM rust:1.67 as builder
WORKDIR /usr/src/celsius-to-fahrenheit
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/celsius-to-fahrenheit /usr/local/bin/celsius-to-fahrenheit
ENTRYPOINT ["celsius-to-fahrenheit"]
