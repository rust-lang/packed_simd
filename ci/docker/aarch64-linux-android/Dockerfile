FROM ubuntu:22.04

RUN dpkg --add-architecture i386 && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
  file \
  make \
  curl \
  ca-certificates \
  python-is-python3 \
  unzip \
  expect \
  openjdk-11-jre \
  libstdc++6:i386 \
  libpulse0 \
  gcc \
  libc6-dev

WORKDIR /android/
COPY android* /android/

ENV ANDROID_ARCH=aarch64

RUN sh /android/android-install-ndk.sh
ENV PATH=$PATH:/android/ndk/toolchains/llvm/prebuilt/linux-x86_64/bin
RUN sh /android/android-install-sdk.sh $ANDROID_ARCH
ENV PATH=$PATH:/android/sdk/cmdline-tools:/android/sdk/platform-tools

RUN mv /root/.android /tmp
RUN chmod 777 -R /tmp/.android
RUN chmod 755 /android/sdk/*tools/**/* /android/sdk/emulator/qemu/linux-x86_64/*

ENV PATH=$PATH:/rust/bin \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER=aarch64-linux-android21-clang \
    CARGO_TARGET_AARCH64_LINUX_ANDROID_RUNNER=/tmp/runtest \
    OBJDUMP=aarch64-linux-android-objdump \
    HOME=/tmp

ADD runtest-android.rs /tmp/runtest.rs
ENTRYPOINT [ \
  "bash", \
  "-c", \
  # set SHELL so android can detect a 64bits system, see
  # http://stackoverflow.com/a/41789144
  "SHELL=/bin/dash /android/sdk/emulator/emulator @aarch64 -no-window & \
   rustc /tmp/runtest.rs -o /tmp/runtest && \
   exec \"$@\"", \
  "--" \
]
