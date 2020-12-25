FROM rust:1 AS builder

# Fetch dependencies
ENV USER=root
WORKDIR /code
RUN cargo init
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch

# Build project
COPY src src
COPY diesel.toml .
COPY migrations migrations
RUN cargo test --offline
RUN cargo build --release --offline

# Build final image
FROM rust:1-slim-buster
RUN apt-get update && \
    apt-get install -qq \
    libpq5
EXPOSE 8090
COPY docs docs
COPY --from=builder /code/target/release/bjoetiek .
CMD ["./bjoetiek"]