FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app
COPY . .

RUN cargo build --release

RUN ldd /app/target/release/spaghetti

CMD ["/app/target/release/spaghetti"]
