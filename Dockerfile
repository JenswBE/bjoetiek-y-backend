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
COPY . .
RUN cargo test
RUN cargo build --target ${TARGET} --release 
RUN cp /usr/src/backend/target/${TARGET}/release/bjoetiek /service

# Build final image
FROM scratch
EXPOSE 8090
COPY --from=builder /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=builder /service service
CMD ["./service"]