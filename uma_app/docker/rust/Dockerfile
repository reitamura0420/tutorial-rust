FROM rust:1.60

WORKDIR /uma_app

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y mariadb-client

RUN cargo install diesel_cli --no-default-features --features mysql