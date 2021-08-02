FROM rust:1.54-slim
WORKDIR .

debian:
    RUN apt-get update
    RUN apt-get install -y gnulib
    RUN cargo install cargo-deb
    COPY Cargo.toml ./
    COPY Cargo.lock ./
    COPY src src
    COPY pkg pkg
    COPY README.md README.md
    RUN cargo deb
    SAVE ARTIFACT /target/debian /debian AS LOCAL target/debian

rpm:
    RUN apt-get update
    RUN apt-get install -y rpm
    RUN cargo install --git https://github.com/iqlusioninc/cargo-rpm
    COPY Cargo.toml ./
    COPY Cargo.lock ./
    COPY src src
    COPY pkg pkg
    COPY README.md README.md
    RUN cargo rpm build
    SAVE ARTIFACT /target/release/rpmbuild /rpm AS LOCAL target/release/rpmbuild
