FROM mcr.microsoft.com/devcontainers/base:bullseye
RUN apt update && apt upgrade -y \
    && apt install -y \
    zip unzip \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

ENV CARGO_BUILD_TARGET_DIR=/tmp/target
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly --no-modify-path
ENV PATH="/root/.cargo/bin:$PATH"

ENV DENO_INSTALL="/root/.deno"
RUN curl -fsSL https://deno.land/x/install/install.sh | sh 
ENV PATH="$DENO_INSTALL/bin:$PATH"