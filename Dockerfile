FROM rust AS builder

# Setup builder
ARG TARGET=x86_64-unknown-linux-musl
RUN apt-get update && \
    apt-get install -qq \
    build-essential \
    musl-tools
WORKDIR /usr/src/backend
RUN rustup target add ${TARGET}

# Build project
COPY src src
COPY Cargo.lock .
COPY Cargo.toml .
COPY diesel.toml .
RUN cargo test
RUN cargo build --target ${TARGET} --release 
RUN mv /usr/src/backend/target/${TARGET}/release/bjoetiek /service

# Build final image
FROM scratch
EXPOSE 8090
COPY docs docs
COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=builder /service service
CMD ["./service"]