FROM rust:latest as builder
WORKDIR /usr/src/oc-scripts
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
COPY --from=builder /usr/local/cargo/bin/oc-scripts /usr/local/bin/oc-scripts
COPY --from=builder /usr/src/oc-scripts/src/static /static
CMD ["oc-scripts"]
