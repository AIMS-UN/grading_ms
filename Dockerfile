FROM ekidd/rust-musl-builder AS builder

ADD . ./
RUN sudo chown -R rust:rust /home/rust/src
RUN cargo build --release
RUN strip /home/rust/src/target/x86_64-unknown-linux-musl/release/aims_grading_ms

FROM debian:buster-slim as runner
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/aims_grading_ms /aims_grading_ms

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
ENTRYPOINT [ "/aims_grading_ms" ]