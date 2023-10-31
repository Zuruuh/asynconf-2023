FROM rust:1.73.0-slim-bookworm as build

WORKDIR /srv

RUN apt update && apt install curl -y

RUN rustup default nightly && \
    rustup target add wasm32-unknown-unknown
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash && \
    cargo binstall trunk --no-confirm

COPY . .

RUN trunk build --release

FROM caddy:2.7.5-alpine as serve

WORKDIR /srv

COPY --from=build /srv/dist /srv/dist

CMD ["caddy", "file-server", "/srv/dist"]

EXPOSE 80
