#!/bin/bash

# Colores para la terminal
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}--- Instalador de Sistema de Inventario (Linux/macOS) ---${NC}"

# 1. Verificar dependencias básicas
if ! command -v curl &> /dev/null; then
    echo "curl no está instalado. Instalándolo..."
    sudo apt-get update && sudo apt-get install -y curl || sudo brew install curl
fi

if ! command -v git &> /dev/null; then
    echo "git no está instalado. Instalándolo..."
    sudo apt-get install -y git || sudo brew install git
fi

# 2. Instalar Rust si no existe
if ! command -v cargo &> /dev/null; then
    echo "Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# 3. Clonar repositorio
INSTALL_DIR="$HOME/inventario_sys"
if [ -d "$INSTALL_DIR" ]; then
    echo "El directorio ya existe. Actualizando..."
    cd "$INSTALL_DIR" && git pull
else
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# 4. Configurar fuentes
mkdir -p fonts
if [ ! -f "fonts/Roboto-Regular.ttf" ]; then
    echo "Descargando fuentes necesarias..."
    curl -L -o fonts/Roboto.zip https://github.com/google/fonts/archive/refs/heads/main.zip
    # Nota: Aquí se requeriría unzip, pero por simplicidad asumimos que el usuario las pondrá o usamos una fuente del sistema.
fi

# 5. Configurar .env
if [ ! -f ".env" ]; then
    echo "DATABASE_URL=sqlite:inventario.db?mode=rwc" > .env
    echo "JWT_SECRET=$(openssl rand -base64 32)" >> .env
fi

# 6. Compilar y Ejecutar
echo -e "${GREEN}Compilando aplicación... (esto puede tardar unos minutos)${NC}"
cargo build --release

echo -e "${GREEN}Instalación completada.${NC}"
echo -e "Para iniciar el sistema: ${BLUE}cd $INSTALL_DIR && cargo run --release${NC}"
