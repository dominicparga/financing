#

FROM docker.io/rust:latest AS builder

ARG MY_APP=billo

WORKDIR /usr/src/${MY_APP}
COPY billo .
RUN cargo install --path .

#

FROM docker.io/ubuntu

ARG USERNAME=billo
ARG MY_APP=billo

# install my app
USER root
COPY --from=builder /usr/local/cargo/bin/${MY_APP} /usr/local/bin/${MY_APP}

# Create user
USER root
RUN useradd --system --create-home --shell /bin/bash ${USERNAME}
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    # && apt-get update \
    # && apt-get install -y sudo \
    # && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    # && chmod 0440 /etc/sudoers.d/$USERNAME

# Let's get it started yeah
USER ${USERNAME}
WORKDIR /workdir

# credits 8)
LABEL maintainer="dominic.parga@gmail.com"
LABEL description="Financing"

ENTRYPOINT [ "billo" ]
