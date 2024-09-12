FROM rust:1.81.0

WORKDIR /app

COPY pokemon.csv /var/lib/mysql-files/

RUN cargo install diesel_cli --no-default-features --features mysql

CMD ["diesel", "setup"]
