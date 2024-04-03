# Get started with a build env with Rust nightly
FROM docker.io/rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN cargo leptos build --release -vv

FROM docker.io/alpine:3.19.1 as runner

WORKDIR /app

COPY --from=builder /work/target/release/music-rs /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/Cargo.toml

RUN mkdir site/assets
VOLUME site/assets

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=./site
EXPOSE 3000

CMD ["/app/music-rs"]