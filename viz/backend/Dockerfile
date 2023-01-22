FROM rust:1.66

WORKDIR /usr/src/app
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY src/ src
COPY .env .env

RUN cargo build --release
RUN chmod +x target/release/viz-backend

EXPOSE 8080

CMD ["./target/release/viz-backend"]