FROM ubuntu:18.04 as gbemulator-dev

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
    && cmake --version

# Clean up
RUN rm -r ~/temp \ 
    && apt-get autoremove -y \
    && apt-get clean -y

# make workdir
RUN mkdir /gb-emulator/ 
WORKDIR /gb-emulator/
COPY . .
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


FROM gbemulator-dev as gbemulator-test

# Configure and build dev target
RUN ./scripts/test-build.sh 
# Run tests
CMD "./build/bin/test"

