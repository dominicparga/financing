FROM docker.io/ubuntu

ARG PY_VERSION="Python 3.10"
ARG USERNAME=billo

USER root
RUN \
# install dependencies
    apt-get update &&\
    apt-get install --yes --no-install-suggests --no-install-recommends \
        python3 python3-pip \
    &&\
    apt-get clean &&\
# check python version
    ((python3 --version | grep "$PY_VERSION" > null) || (echo "ERROR: Expected $PY_VERSION, actual $(python3 --version)" && exit 310))

# Create the user
USER root
RUN useradd --system --create-home --shell /bin/bash ${USERNAME}
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    # && apt-get update \
    # && apt-get install -y sudo \
    # && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    # && chmod 0440 /etc/sudoers.d/$USERNAME

USER ${USERNAME}
WORKDIR /workdir
COPY billo/ ./billo/

# credits 8)
LABEL maintainer="dominic.parga@gmail.com"
LABEL description="Financing"

ENTRYPOINT [ "python3", "-B", "-m", "billo" ]
