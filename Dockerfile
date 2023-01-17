FROM rust:alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR build .
COPY . .

RUN cargo install --path .

FROM alpine:latest
COPY --from=builder /usr/local/cargo/bin/neuters /usr/local/bin/neuters

CMD ["neuters", "--address", "0.0.0.0:13369"]
