FROM rust:1.66

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["cargo", "run"]