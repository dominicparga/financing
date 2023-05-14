FROM docker.io/dominicparga/devcontainer:latest

ARG PY_VERSION="Python 3.10"

USER root
RUN \
# install dependencies
    apt-get update &&\
    apt-get install --yes --no-install-suggests --no-install-recommends \
        # developing py
        python3 python3-pip \
        # installing Rust
        curl \
    &&\
    apt-get clean &&\
# check python version
    ((python3 --version | grep "$PY_VERSION" > null) || (echo "ERROR: Expected $PY_VERSION, actual $(python3 --version)" && exit 310))

USER vscode
RUN \
# install Rust
    curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path &&\
# install py dependencies
    pip install \
        # testing
        pytest \
        # linting
        pylint \
        # formatting, stricter than autopep8
        black \
        isort
ENV PATH="/home/vscode/.cargo/bin:${PATH}"
# RUN echo 'source ${HOME}/.cargo/env' >> ${HOME}/.bashrc

LABEL maintainer="dominic.parga@gmail.com"
LABEL description="Financing"
