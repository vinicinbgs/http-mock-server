FROM rust:1.69 as builder

RUN mkdir /usr/src/app

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

CMD [ "./target/release/rust_project" ]