# syntax = docker/dockerfile:1.2
FROM rust:slim

RUN apt update
RUN apt install -y pkg-config libssl-dev curl git sudo
RUN mkdir grpcurl && \
    cd grpcurl && \
    curl -L https://github.com/fullstorydev/grpcurl/releases/download/v1.8.5/grpcurl_1.8.5_linux_x86_64.tar.gz -o grpcurl.tar.gz && \
    tar -zxvf grpcurl.tar.gz && \
    mv grpcurl /usr/local/bin && \
    cd ~ && \
    rm -rf grpcurl
ARG USERNAME=app
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
USER $USERNAME
WORKDIR /home/$USERNAME
RUN rustup component add rustfmt
