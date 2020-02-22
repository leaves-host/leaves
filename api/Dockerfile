FROM rust:1.41 as build

WORKDIR /app

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release

RUN rm -f target/release/deps/leaves*
COPY src ./src

RUN cargo build --release

FROM rust:1.41

WORKDIR /app

COPY --from=build /app/target/release/leaves ./leaves

ENTRYPOINT ./leaves
