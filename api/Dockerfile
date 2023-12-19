FROM rust:latest as build

RUN USER=root cargo new --bin reminders
WORKDIR /reminders

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Cache deps
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/reminders*
RUN cargo build --release


FROM rust:latest

COPY --from=build /reminders/target/release/reminders .

CMD ["./reminders"]
