# Build the binary
FROM rust:1.77.1 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Bookworm is the same base image rust uses for its official images
# pull it in and only copy over binary for a lightweight image
FROM debian:bookworm-slim
WORKDIR /usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3000

COPY --from=builder /usr/src/app/target/release/project-suggestions ./app

CMD ["./app"]