FROM rust:latest AS build
WORKDIR /usr/src/serenity_template
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
COPY --from=build /usr/local/cargo/bin/rusty /usr/local/bin/rusty
RUN apt-get update && apt-get install -y ca-certificates
CMD ["rusty"]
