FROM rust:latest as build

RUN cargo install trunk

RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

COPY . .

RUN trunk build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates glibc-source libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/src/app/dist /usr/app/dist

WORKDIR /usr/app

EXPOSE 8080

CMD ["trunk", "serve"]
