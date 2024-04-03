FROM rust:latest
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config
RUN cargo build --release
EXPOSE 8080
ENTRYPOINT [ "/app/target/release/test_actix" ]

