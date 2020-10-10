FROM rust:1.47.0 as builder

RUN mkdir /app
WORKDIR /app

ENV RUST_BACKTRACE full
ENV RUSTUP_TOOLCHAIN nightly

ADD rust-toolchain .
ADD Cargo.toml .
ADD Cargo.lock .
ADD log4rs.yml .
COPY dummy.rs .
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
ADD src ./src
ADD templates ./templates
RUN cargo build --release

CMD /app/target/release/rust-wiki

################################################

FROM debian:stretch-slim

RUN mkdir /app
WORKDIR /app

COPY --from=builder /app/target/release/rust-wiki .
COPY --from=builder /app/templates ./templates

CMD /app/rust-wiki