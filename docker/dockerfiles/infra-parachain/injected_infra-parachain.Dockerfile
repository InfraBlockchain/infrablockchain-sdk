FROM docker.io/library/ubuntu:22.04

COPY ./target/release/polkadot-parachain /usr/local/bin

RUN apt-get update
RUN apt-get install -y ca-certificates
RUN update-ca-certificates

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/polkadot-parachain"]
