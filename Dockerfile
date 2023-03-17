# Build Stage
FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /build 
COPY . .

RUN cargo install --path .

# Final Image
FROM gcr.io/distroless/cc

COPY --from=builder /usr/local/cargo/bin/neuters /

CMD ["./neuters", "--address", "0.0.0.0:13369"]
