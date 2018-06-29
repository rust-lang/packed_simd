FROM ubuntu:18.10

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    cmake \
    gcc \
    libc6-dev \
    make \
    pkg-config \
    g++-mipsel-linux-gnu \
    libc6-dev-mipsel-cross \
    qemu-user \
    qemu-system-mips \
    file

ENV CARGO_TARGET_MIPS_UNKNOWN_LINUX_GNU_LINKER=mipsel-linux-gnu-gcc \
    CARGO_TARGET_MIPS_UNKNOWN_LINUX_GNU_RUNNER="qemu-mipsel -L /usr/mipsel-linux-gnu" \
    CC_mipsel_unknown_linux_gnu=mipsel-linux-gnu-gcc \
    CXX_mipsel_unknown_linux_gnu=mipsel-linux-gnu-g++ \
    OBJDUMP=mipsel-linux-gnu-objdump