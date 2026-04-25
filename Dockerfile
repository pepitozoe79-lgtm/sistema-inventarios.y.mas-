# --- Stage 1: Builder ---
FROM rust:1.75-slim-bookworm as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binutils \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

RUN cargo build --release
# Optimizar binario (eliminar símbolos)
RUN strip target/release/inventario_sys

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Crear usuario no-root por seguridad
RUN useradd -m -u 1000 appuser

WORKDIR /app

# Crear carpeta para datos con permisos correctos
RUN mkdir -p /app/data && chown -R appuser:appuser /app/data

COPY --from=builder --chown=appuser:appuser /app/target/release/inventario_sys .
COPY --from=builder --chown=appuser:appuser /app/static ./static
COPY --from=builder --chown=appuser:appuser /app/fonts ./fonts

USER appuser

EXPOSE 3000

ENV DATABASE_URL=sqlite:/app/data/inventario.db?mode=rwc
ENV RUST_LOG=info

# Monitor de salud
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl --fail http://localhost:3000/api/v1/health || exit 1

CMD ["./inventario_sys"]
