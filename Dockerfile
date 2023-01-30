# =================== STAGE 0 =================== 
# Build qemu with patches
from ubuntu:20.04

arg SUPPORTED_QEMU_SHA256=00b1faea41d283e931256aa78aa975a369ec3ae6
arg TARGET_LIST="mipsel-linux-user,riscv64-linux-user,arm-linux-user,x86_64-linux-user,i386-linux-user"

# Get qemu build deps as suggested here:
#  https://wiki.qemu.org/Hosts/Linux
env DEBIAN_FRONTEND=none
run apt update && \
    apt install -y \
        gcc \
        make \
        git libglib2.0-dev libfdt-dev libpixman-1-dev zlib1g-dev ninja-build \
        git-email \
        libaio-dev libbluetooth-dev libcapstone-dev libbrlapi-dev libbz2-dev \
        libcap-ng-dev libcurl4-gnutls-dev libgtk-3-dev \
        libibverbs-dev libjpeg8-dev libncurses5-dev libnuma-dev \
        librbd-dev librdmacm-dev \
        libsasl2-dev libsdl2-dev libseccomp-dev libsnappy-dev libssh-dev \
        libvde-dev libvdeplug-dev libvte-2.91-dev libxen-dev liblzo2-dev \
        valgrind xfslibs-dev \
        libnfs-dev libiscsi-dev

# Get qemu source and apply cannoli patches
env QEMU_DIR="/build/qemu"
workdir ${QEMU_DIR}
run git init && \
    git remote add origin https://gitlab.com/qemu-project/qemu.git && \
    git fetch --depth 1 origin ${SUPPORTED_QEMU_SHA256} && \
    git checkout FETCH_HEAD 
copy ./qemu_patches.patch .
run git \
    -c user.name="cannoli" \
    -c user.email="c@nno.li" \
    am --3way ./qemu_patches.patch

# Configure and build qemu with the requested arches
env CANNOLI_DIR="/build/cannoli"
workdir ${CANNOLI_DIR}/jitter/ffi/
copy ./jitter/ffi/cannoli.h .
workdir ${QEMU_DIR}
run ./configure \
        --target-list=${TARGET_LIST} \
        --extra-ldflags="-ldl" \
        --with-cannoli=${CANNOLI_DIR} \
        --static && \
    make -j $(nproc)

# Rename the resulting qemu bins
run for x in $(\
        find "${QEMU_DIR}/build/" \
            -maxdepth 1 \
            -name "qemu-*" \
            -type f \
            -exec file {} \; | \
        grep -oP '(^.*)(?=: ELF)' \
    ); do cp $x "$x-static"; done


# =================== STAGE 1 =================== 
# Start a new container without all the qemu cruft for cannoli
from ubuntu:20.04

run apt update && \
    apt install -y \
        curl \
        build-essential \
        clang

# Copy in the previously built qemu files
env QEMU_DIR=/build/qemu
workdir ${QEMU_DIR}
copy --from=0 ${QEMU_DIR}/build/qemu-*-static ./

# Install rust
run curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

# Build cannoli
env CANNOLI_DIR=/build/cannoli
workdir ${CANNOLI_DIR}
copy . .
run bash -c "source ~/.cargo/env && cargo build --release"
