FROM ubuntu:18.04

# Global settings
ENV CMAKE_VERSION=3.17.2

# Pre-requsities and development tools.
RUN apt-get update \
    && apt-get install -y software-properties-common \
    unzip wget git \
    build-essential gdb clang clang-format clang-tidy

# Install CMake binary
RUN mkdir -p ~/temp \
    && cd ~/temp \
    && wget https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-Linux-x86_64.sh \
    && mkdir /opt/cmake \
    && sh cmake-$CMAKE_VERSION-Linux-x86_64.sh --prefix=/opt/cmake --skip-license \
    && ln -s /opt/cmake/bin/cmake /usr/local/bin/cmake \
    && cmake --version \

    # emscripten
    && cd ~ && git clone https://github.com/emscripten-core/emsdk.git && cd emsdk \
    && ./emsdk install latest \
    && ./emsdk activate latest \
    && echo "source \"/home/${USERNAME}/emsdk/emsdk_env.sh\"" >> /home/${USERNAME}/.bashrc \
    && cd ~ \

    # OpenGL - (glvnd)
    RUN apt-get update && apt-get install -y --no-install-recommends \
    libxau6 \
    libxdmcp6 \
    libxcb1  \
    libxext6 \
    libx11-6 \
    pkg-config \
    libglvnd-dev \
    libgl1-mesa-dev \
    libegl1-mesa-dev \
    libgles2-mesa-dev \
    ligsdl2-dev \
    mesa-utils \
    && rm -rf /var/lib/apt/lists/*

# Deps
RUN apt-get update \
    && apt-get install -y libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev


ENV NVIDIA_DRIVER_CAPABILITIES \
    ${NVIDIA_DRIVER_CAPABILITIES:+$NVIDIA_DRIVER_CAPABILITIES,}graphics,compat32

# Install docker
RUN apt-get update \
    && apt-get -y install git gcc curl zip unzip apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common \
    # Install Docker - x86_64 / amd64 stable
    && curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add - \
    && add-apt-repository \
    "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) \
    stable" \
    && apt-get update \
    && apt-get -y install docker-ce docker-ce-cli containerd.io

# act to test actions: https://github.com/nektos/act
RUN curl https://raw.githubusercontent.com/nektos/act/master/install.sh | bash

# Clean up
RUN rm -r ~/temp \ 
    && apt-get autoremove -y \
    && apt-get clean -y

# Configure a SUDO non-root user (no password)
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME