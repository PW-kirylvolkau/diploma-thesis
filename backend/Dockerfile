# copied from here: https://betterprogramming.pub/how-to-deploy-a-rust-web-server-to-heroku-using-axum-docker-and-github-actions-6cddb442ea7e
FROM rust:1.66 as build

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.env ./.env
COPY ./sqlx-data.json ./sqlx-data.json
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update 
RUN apt-get -y install libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=build /backend/target/release/backend .

# RUN cargo install sqlx-cli
# RUN cargo sqlx prepare

CMD ["./backend"]
EXPOSE 3000