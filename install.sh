#!/bin/bash

# Colores para la terminal
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}--- Instalador Inteligente: Inventario Pro (Linux) ---${NC}"

# 1. Verificar herramientas de compilación
if ! command -v gcc &> /dev/null || ! command -v make &> /dev/null; then
    echo -e "${YELLOW}⚠️ ERROR: No se detectaron las herramientas de compilación (gcc/make).${NC}"
    echo -e "Rust necesita 'build-essential' para compilar el sistema."
    echo ""
    
    # Detectar gestor de paquetes y sugerir comando
    if command -v apt &> /dev/null; then
        echo -e "Ejecuta: ${GREEN}sudo apt update && sudo apt install build-essential -y${NC}"
    elif command -v dnf &> /dev/null; then
        echo -e "Ejecuta: ${GREEN}sudo dnf groupinstall \"Development Tools\"${NC}"
    elif command -v pacman &> /dev/null; then
        echo -e "Ejecuta: ${GREEN}sudo pacman -S base-devel${NC}"
    else
        echo -e "Por favor, instala el paquete de herramientas de desarrollo de tu distribución."
    fi
    exit 1
fi

# 2. Clonar repositorio
TARGET_DIR="$HOME/Documents/inventario_pro"
if [ -d "$TARGET_DIR" ]; then
    echo -e "Actualizando repositorio existente en $TARGET_DIR..."
    cd "$TARGET_DIR" && git pull
else
    echo -e "Clonando repositorio..."
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- "$TARGET_DIR"
    cd "$TARGET_DIR"
fi

# 3. Compilar
echo -e "${CYAN}Compilando aplicación (esto puede tardar unos minutos)...${NC}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ ¡Instalación completada con éxito!${NC}"
    echo -e "Para iniciar el sistema, ejecuta:"
    echo -e "${YELLOW}cd $TARGET_DIR && cargo run --release${NC}"
else
    echo -e "${RED}❌ Hubo un error durante la compilación.${NC}"
fi
