FROM ubuntu:20.04
WORKDIR /home/api

RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get -y install build-essential curl

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

CMD ["cargo", "watch", "-x", "run"]