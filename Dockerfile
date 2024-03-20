FROM rust:1.70 AS build-container

# setup dummie projet
RUN USER=root cargo new build_dir
WORKDIR /build_dir

# coping and installing the dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

# coping and build base code
COPY src ./src
RUN cargo build --release

FROM debian:bullseye-slim

# Set the MONGOURI environment variable
ENV MONGOURI mongodb://root:example@mongo:27017/

COPY --from=build-container /build_dir/target/release/rust-api-mongodb .

RUN apt update && apt install libssl-dev ca-certificates -y

# Expose both HTTP and HTTPS ports
EXPOSE 8080

CMD ["./rust-api-mongodb"]