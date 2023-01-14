# copied from here: https://betterprogramming.pub/how-to-deploy-a-rust-web-server-to-heroku-using-axum-docker-and-github-actions-6cddb442ea7e
FROM rust:1.66

WORKDIR /backend
COPY . .
RUN cargo install --path .

CMD ["backend"]
EXPOSE 3000