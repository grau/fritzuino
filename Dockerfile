FROM rust:1-alpine as builder
WORKDIR /bell-server
COPY bell-server/ .
RUN cargo build --release

FROM alpine:latest  
WORKDIR /bell-server
COPY --from=builder /bell-server/target/release/bell-server .
ENV RUST_LOG=info
CMD ["./bell-server", "-vv"]
