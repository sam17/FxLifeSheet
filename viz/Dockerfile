FROM rust:1.61.0 as rust-builder

WORKDIR /usr/src/app
COPY backend/Cargo.lock Cargo.lock
COPY backend/Cargo.toml Cargo.toml
COPY backend/src src
COPY backend/.env .env

RUN cargo build --release
RUN chmod +x target/release/viz-backend


FROM node:latest AS react-build
WORKDIR /build

COPY frontend/package.json package.json
COPY frontend/package-lock.json package-lock.json
COPY frontend/tsconfig.json tsconfig.json
RUN npm ci

COPY frontend/public/ public
COPY frontend/src/ src
RUN npm run build


FROM debian:buster-slim
ARG APP=/usr/src/app
WORKDIR ${APP}

RUN mkdir -p ${APP}/backend && mkdir -p ${APP}/frontend/build

EXPOSE 8080

COPY --from=rust-builder /usr/src/app/target/release/viz-backend ${APP}/backend/viz-backend
COPY --from=rust-builder /usr/src/app/.env ${APP}/backend/.env
COPY --from=react-build /build/build/ ${APP}/frontend/build

WORKDIR ${APP}/backend
CMD ["./viz-backend"]