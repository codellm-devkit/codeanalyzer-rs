FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Arguments for user creation
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=1000

# Install necessary packages
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        sudo \
        sed \
        vim \
        git \
        make \
        gcc \
        zlib1g-dev \
        zip \
        unzip \
        tree \
        curl \
        wget \
        jq \
        build-essential \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Create the user with sudo privileges
RUN groupadd -f --gid $USER_GID $USERNAME \
    && if id "$USERNAME" &>/dev/null; then echo "User $USERNAME already exists."; else useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash; fi \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

WORKDIR /codeanalyzer-rs

# Switch to the vscode user
USER $USERNAME

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup install stable \
    && rustup default stable \
    && rustup component add rustfmt clippy

# Update PATH for Rust
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"