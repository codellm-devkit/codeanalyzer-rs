FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

# Arguments for user creation
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Install necessary packages
# Combine RUN commands and clean up in the same layer to reduce image size
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
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME -s /bin/bash \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Set up Rust for the vscode user
USER $USERNAME
WORKDIR /workspaces/codeanalyzer-rs

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && . $HOME/.cargo/env \
    && rustup install nightly \
    && rustup default nightly \
    && rustup component add rustfmt clippy

# Update PATH for Rust
ENV PATH="/home/${USERNAME}/.cargo/bin:${PATH}"

# Pre-build common Rust tools to speed up container startup
RUN cargo install cargo-watch cargo-edit