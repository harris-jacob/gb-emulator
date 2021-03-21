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
COPY . /gb-emu/


FROM gbemulator-dev as gbemulator-test

# Configure and build dev target
RUN cd gb-emu/ \
    && /gb-emu/scripts/test-build.sh

CMD ["/gb-emu/build/bin/test"]

