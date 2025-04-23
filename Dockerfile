FROM rust:1.74

WORKDIR /app

# Instalar dependencias para sqlx
RUN apt-get update && apt-get install -y pkg-config libpq-dev

COPY  . .

RUN cargo build --release

CMD ["./target/release/rust-project-logistic"]