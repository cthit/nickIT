FROM debian
COPY ["Cargo.toml", "Rocket.toml", "/nickit-backend/"]
COPY ["./src"   , "/nickit-backend/src/"]
WORKDIR /nickit-backend/
RUN apt-get update && apt-get install -y curl gcc pkg-config libssl-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly && \
    /root/.cargo/bin/cargo build --release && \
    groupadd -r nickit && \
    useradd --no-log-init -r -g nickit nickit && \
    chown -R nickit:nickit /nickit-backend
USER nickit
CMD ["./target/release/nickit_backend"]
