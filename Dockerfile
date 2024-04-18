# Build the UI
FROM node:16 as ui-builder
WORKDIR /usr/src/app
COPY ./ui .
COPY ./ui/.env ./.env.production
RUN npm install
RUN npm run build

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

EXPOSE 5000

COPY --from=builder /usr/src/app/target/release/project-suggestions ./app
COPY --from=ui-builder /usr/src/dist ./dist

CMD ["./app"]