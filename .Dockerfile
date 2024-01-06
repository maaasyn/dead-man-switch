# docker push maaasyn/dead-man-switch:tagname

FROM rust:1.75 as builder
WORKDIR /usr/src

COPY . .

RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/target/release/my-app /usr/local/bin

CMD ["my-app"]