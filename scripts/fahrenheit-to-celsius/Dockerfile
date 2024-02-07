FROM rust:1.67 as builder
WORKDIR /usr/src/fahrenheit-to-celsius
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/fahrenheit-to-celsius /usr/local/bin/fahrenheit-to-celsius
ENTRYPOINT ["fahrenheit-to-celsius"]
