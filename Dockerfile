FROM rust:bullseye

RUN cargo install cargo-compete \
    && rustup install 1.70.0 \
    && rustup default 1.70.0
