FROM ubuntu:20.04
WORKDIR /home/api

# Update and install some required packages.
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get -y install build-essential curl

# Install the Rust toolchain.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Cargo Watch and SQLx CLI.
RUN cargo install cargo-watch
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres

# Run the server.
CMD ["cargo", "watch", "-x", "run"]