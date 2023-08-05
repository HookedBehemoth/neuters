# Build Stage
FROM rust:alpine as builder

RUN apk add --no-cache musl-dev git

WORKDIR /build 
COPY . .

RUN cargo install --path .

# Final Image
FROM gcr.io/distroless/static

COPY --from=builder /usr/local/cargo/bin/neuters /

CMD ["./neuters", "--address", "0.0.0.0:13369"]
