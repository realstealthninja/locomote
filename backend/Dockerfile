FROM rust:1.60

WORKDIR /app

COPY Cargo.toml .

RUN cargo build

COPY target/release/* .

EXPOSE 8080

CMD ["./main"]