# ═══════════════════════════════════════════════════════════════════════════
# TESSERAX PROTOCOL - DOCKER BUILD
# ═══════════════════════════════════════════════════════════════════════════
# Multi-stage build for optimized production image
# 
# Features:
#   - Post-Quantum Security (CRYSTALS-Dilithium Level 2)
#   - Re-ML System (STARK-based signature compression)
#   - Full EVM Compatibility (Frontier)
#   - Sigmoid Emission Schedule
#   - Chain ID: 13817 (derived from π × e × φ × 10^6)
# ═══════════════════════════════════════════════════════════════════════════

# Stage 1: Build
FROM docker.io/paritytech/ci-unified:bullseye-1.77.0 as builder

LABEL maintainer="Tesserax Protocol <team@tesserax.network>"
LABEL description="Tesserax Protocol Node - Build Stage"

WORKDIR /tesserax

# Copy source code
COPY . /tesserax

# Build release binary
RUN cargo fetch && \
    cargo build --locked --release && \
    # Verify binary
    /tesserax/target/release/tesserax-node --version

# Stage 2: Runtime
FROM docker.io/parity/base-bin:latest

LABEL maintainer="Tesserax Protocol <team@tesserax.network>"
LABEL description="Tesserax Protocol Node - Quantum-Resistant Blockchain"
LABEL org.opencontainers.image.source="https://github.com/Tesserax-Protocol/tesserax-node"
LABEL org.opencontainers.image.documentation="https://tesserax.network/docs"
LABEL org.opencontainers.image.licenses="MIT"

# Copy the compiled binary from builder
COPY --from=builder /tesserax/target/release/tesserax-node /usr/local/bin/tesserax-node

# Setup non-root user and directories
USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /tesserax tesserax && \
    mkdir -p /data /tesserax/.local/share && \
    chown -R tesserax:tesserax /data && \
    ln -s /data /tesserax/.local/share/tesserax && \
    # Verify binary works
    /usr/local/bin/tesserax-node --version

# Switch to non-root user
USER tesserax

# Exposed Ports:
#   30333 - P2P networking (libp2p)
#   9933  - HTTP RPC (legacy, use 9944 instead)
#   9944  - WebSocket RPC (Substrate + Ethereum JSON-RPC)
#   9615  - Prometheus metrics
EXPOSE 30333 9933 9944 9615

# Persistent data volume
VOLUME ["/data"]

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
  CMD /usr/local/bin/tesserax-node --version || exit 1

ENTRYPOINT ["/usr/local/bin/tesserax-node"]

# Default command (can be overridden)
CMD ["--dev"]
