FROM rust:1.31

RUN mkdir /usr/src/app

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

RUN cargo build

CMD ["cargo", "run"]