FROM debian

COPY ["Cargo.toml", "Rocket.toml", "/nickit-api/"]
COPY ["./src"   , "/nickit-api/src/"]

WORKDIR /nickit-api/

# Install dependencies
RUN apt-get update && apt-get install -y curl gcc pkg-config libssl-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \
    groupadd -r nickit && useradd --no-log-init -r -g nickit nickit

# Compile program
RUN PATH=$PATH:$HOME/.cargo/bin && \
    cargo build --release && \
    mv ./target/release/nickit_api . && \
    cargo clean && \
    chown -R nickit:nickit /nickit-api

USER nickit
CMD ["./nickit_api"]
