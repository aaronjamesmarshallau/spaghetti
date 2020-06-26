FROM rustlang/rust:nightly AS build

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /app
COPY --from=build /app/target/release/spaghetti .

RUN apt-get update && apt-get install -y libpq-dev

CMD ["/app/spaghetti"]
