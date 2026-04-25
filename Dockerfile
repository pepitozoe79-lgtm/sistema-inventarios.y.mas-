# --- Stage 1: Builder ---
FROM rust:1.75-slim-bookworm as builder

# Instalar dependencias de compilación
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Compilar en modo release
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Instalar dependencias mínimas de ejecución y certificados
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Crear carpeta para datos persistentes
RUN mkdir -p /app/data

# Copiar el binario desde el builder
COPY --from=builder /app/target/release/inventario_sys .

# Copiar archivos estáticos y fuentes
COPY --from=builder /app/static ./static
COPY --from=builder /app/fonts ./fonts

# Exponer el puerto
EXPOSE 3000

# Variables de entorno por defecto
ENV DATABASE_URL=sqlite:/app/data/inventario.db?mode=rwc
ENV RUST_LOG=info

# Comando de inicio
CMD ["./inventario_sys"]
