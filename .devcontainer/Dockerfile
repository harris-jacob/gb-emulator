FROM nvidia/cuda:10.2-cudnn7-devel-ubuntu18.04

# global settings
ENV CMAKE_VERSION=3.17.2

# configure a sudo non-root user (no password)
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME
USER $USERNAME

# Pre-requsities and development tools.
RUN sudo apt-get update \
    && sudo apt-get install -y software-properties-common \
    zip unzip wget git gcc curl \
    apt-transport-https ca-certificates gnupg-agent \
    build-essential gdb clang clang-format clang-tidy \
    libsdl2-dev mesa-utils

# cmake
RUN mkdir -p ~/temp && cd ~/temp \
    && wget https://github.com/Kitware/CMake/releases/download/v$CMAKE_VERSION/cmake-$CMAKE_VERSION-Linux-x86_64.sh \
    && sudo mkdir /opt/cmake \
    && sudo sh cmake-$CMAKE_VERSION-Linux-x86_64.sh --prefix=/opt/cmake --skip-license \
    && sudo ln -s /opt/cmake/bin/cmake /usr/local/bin/cmake \
    && cd ~ && cmake --version 

# emscripten
RUN cd ~ && git clone https://github.com/emscripten-core/emsdk.git && cd emsdk \
    && ./emsdk install latest \
    && ./emsdk activate latest \
    && echo "source \"/home/${USERNAME}/emsdk/emsdk_env.sh\"" >> /home/${USERNAME}/.bashrc \
    && cd ~ 

# Install docker
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add - \
    && sudo add-apt-repository \
    "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
    $(lsb_release -cs) \
    stable" \
    && sudo apt-get update \
    && sudo apt-get -y install docker-ce docker-ce-cli containerd.io


# nodejs & yarn
RUN mkdir -p ~/temp && cd ~/temp \
    && sudo curl -sL https://deb.nodesource.com/setup_14.x | sudo bash - \
    && sudo apt-get install -y nodejs \
    && sudo curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | sudo apt-key add - \
    && echo "deb https://dl.yarnpkg.com/debian/ stable main" | sudo tee /etc/apt/sources.list.d/yarn.list \
    && sudo apt-get update && sudo apt-get install -y yarn \
    && cd ~ 