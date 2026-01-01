# ═══════════════════════════════════════════════════════════════════════════
# TESSERAX PROTOCOL - DOCKER BUILD
# ═══════════════════════════════════════════════════════════════════════════
# Multi-stage build for optimized production image
# 
# Features:
#   - Post-Quantum Security (CRYSTALS-Dilithium)
#   - Full EVM Compatibility (Frontier)
#   - Sigmoid Emission Schedule
#   - Chain ID: 13817 (derived from π × e × φ × 10^6)
# ═══════════════════════════════════════════════════════════════════════════

# Stage 1: Build
FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /tesserax
COPY . /tesserax

# Fetch dependencies and build release binary
RUN cargo fetch
RUN cargo build --locked --release

# Stage 2: Runtime
FROM docker.io/parity/base-bin:latest

# Copy the compiled binary
COPY --from=builder /tesserax/target/release/tesserax-node /usr/local/bin

# Setup user and directories
USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /tesserax tesserax && \
	mkdir -p /data /tesserax/.local/share && \
	chown -R tesserax:tesserax /data && \
	ln -s /data /tesserax/.local/share/tesserax && \
	# Minimize attack surface
	rm -rf /usr/bin /usr/sbin && \
	# Verify binary works
	/usr/local/bin/tesserax-node --version

USER tesserax

# Ports:
#   30333 - P2P networking
#   9933  - Legacy HTTP RPC (deprecated)
#   9944  - WebSocket RPC (Substrate + Ethereum JSON-RPC)
#   9615  - Prometheus metrics
EXPOSE 30333 9933 9944 9615

VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/tesserax-node"]
