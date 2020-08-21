FROM rust:alpine
RUN apk add --update --no-cache build-base nodejs npm curl
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
