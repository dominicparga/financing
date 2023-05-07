FROM docker.io/dominicparga/devcontainer:latest

USER root
RUN apt-get update &&\
    apt-get install --yes --no-install-suggests --no-install-recommends \
        curl \
    &&\
    apt-get clean

USER vscode
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path
ENV PATH="/home/vscode/.cargo/bin:${PATH}"
# RUN echo 'source ${HOME}/.cargo/env' >> ${HOME}/.bashrc
