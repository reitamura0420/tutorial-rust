FROM rust:1.60

WORKDIR /three_architecture

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install -y mariadb-client

RUN cargo install diesel_cli --no-default-features --features mysql