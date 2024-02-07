FROM rust:latest as builder
WORKDIR /usr/src/oc-scripts
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/oc-scripts /usr/local/bin/oc-scripts
CMD ["oc-scripts"]
