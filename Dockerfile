# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /app

COPY . .

RUN cargo build -j 12 --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM ubuntu

COPY --from=cargo-build /app/target/release/pls-give /app/pls-give

RUN apt-get update -y && apt install libssl-dev -y

ENTRYPOINT ["/app/pls-give"]
