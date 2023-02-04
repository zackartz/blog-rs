FROM rustlang/rust:nightly-bullseye-slim

RUN apt update && apt install libssl-dev pkg-config -y

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos

COPY . .

RUN cargo leptos build --release

CMD ["cargo", "leptos", "serve"]
