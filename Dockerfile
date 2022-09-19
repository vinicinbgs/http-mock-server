FROM rust:1.63 as builder
RUN mkdir /usr/src/app
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
EXPOSE 7878/tcp
CMD [ "./target/release/rust_project" ]