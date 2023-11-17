FROM ubuntu:22.04

# Install dependencies
RUN apt update && \
    apt install -y git nfs-common && \
    rm -rf /var/lib/apt/lists/*

# Copy binaries for git-xet and volume binary
RUN git config --global user.email "operations+k8s@xethub.com"  && \
    git config --global user.name "xet"

COPY ./rust/target/release/git-xet /usr/local/bin/git-xet
COPY ./rust/target/release/k8s-csi-xetfs k8s-csi-xetfs

ENV HOME=/home

ENTRYPOINT ["/k8s-csi-xetfs"]
