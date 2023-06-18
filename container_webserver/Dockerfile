FROM rust:1.70 as builder
RUN USER=root cargo new --bin workdir
WORKDIR /workdir
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN rm src/*rs
COPY ./src ./src
RUN cargo build --release


FROM debian:buster-slim as runtime
EXPOSE 3000
COPY --from=builder /workdir/target/release/container_webserver ./
CMD ["./container_webserver"]