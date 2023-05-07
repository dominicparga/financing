FROM docker.io/dominicparga/devcontainer:latest

ARG PY_VERSION="Python 3.10"

# USER root
# RUN \
# # install dependencies
#     apt-get update &&\
#     apt-get install --yes --no-install-suggests --no-install-recommends \
#         python3 python3-pip \
#     &&\
#     apt-get clean &&\
# # check python version
#     ((python3 --version | grep "$PY_VERSION" > null) || (echo "ERROR: Expected $PY_VERSION, actual $(python3 --version)" && exit 310))

# USER vscode

# RUN \
#     pip install \
#         # testing
#         pytest \
#         # linting
#         pylint \
#         # formatting, stricter than autopep8
#         black \
#         isort

LABEL maintainer="dominic.parga@gmail.com"
LABEL description="Financing"
