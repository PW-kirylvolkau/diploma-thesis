# copied from here: https://betterprogramming.pub/how-to-deploy-a-rust-web-server-to-heroku-using-axum-docker-and-github-actions-6cddb442ea7e
FROM rust:1.64 as build

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /backend/target/release/backend .

CMD ["./backend"]