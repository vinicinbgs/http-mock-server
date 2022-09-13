# Builder
FROM rust:1.63 as builder
RUN mkdir /usr/src/app
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runner
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust_project /usr/local/bin/rust_project
WORKDIR /usr/local/bin
EXPOSE 7878/tcp
CMD [ "./rust_project" ]