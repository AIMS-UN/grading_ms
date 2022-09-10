FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/aims_grading_ms /usr/local/bin/aims_grading_ms
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
ENTRYPOINT [ "aims_grading_ms" ]