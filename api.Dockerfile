FROM rust:1.48 as build

WORKDIR /leaves/api

COPY ./models ../models
RUN echo "[workspace]\nmembers = ['api', 'models']" > ../Cargo.toml
COPY ./api/Cargo.toml ./Cargo.toml

RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs
RUN cargo build --bin leaves-api --release

RUN rm -f ../target/release/deps/leaves*
RUN rm -r ./src
COPY ./api/src ./src

RUN cargo build --release

FROM rust:1.48

WORKDIR /app

COPY --from=build /leaves/target/release/leaves-api ./leaves-api

VOLUME /data

ENTRYPOINT ./leaves-api
