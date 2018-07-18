FROM ubuntu:17.10
RUN apt-get update && apt-get install -y --no-install-recommends \
  gcc \
  ca-certificates \
  libc6-dev \
  gcc-arm-linux-gnueabi \
  libc6-dev-arm-cross \
  qemu-user \
  make \
  file
ENV CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabi-gcc \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_RUNNER="qemu-arm -L /usr/arm-linux-gnueabi" \
    OBJDUMP=arm-linux-gnueabi-objdump
