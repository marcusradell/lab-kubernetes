FROM rust:alpine3.21

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN apk add --no-cache musl-dev gcc
RUN cargo build --release
RUN cargo build --release --bin api



RUN cargo install --path .

CMD ["api"]