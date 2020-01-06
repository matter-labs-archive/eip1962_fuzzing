FROM rust:latest
WORKDIR /fuzz
COPY . .
RUN apt-get update 
RUN apt-get -y install build-essential binutils-dev libunwind-dev libblocksruntime-dev
RUN cargo install honggfuzz
CMD ["/bin/bash", "docker_run_honggfuzz.sh"]