FROM rust:buster as builder
RUN apt-get update -y && apt-get install -y clang libclang-dev
COPY . /src
WORKDIR /src
RUN cargo build --release --no-default-features --features rocksdb-storage

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /src/target/release/xqd /bin/xqd
VOLUME /db
CMD ["/bin/xqd", "-d", "/db"]
