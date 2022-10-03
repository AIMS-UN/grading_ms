FROM rust AS builder
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY dummy.rs .
RUN sed -i 's#src/main.rs#dummy.rs#' ./Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' ./Cargo.toml

COPY ./src ./src
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/aims_grading_ms .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000
CMD ["./aims_grading_ms"]

