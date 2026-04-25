#!/bin/sh

# Colores (compatibles con printf)
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

printf "${CYAN}--- Instalador Universal: Inventario Pro (Linux) ---${NC}\n"

# 1. Verificar herramientas de compilación (POSIX compatible)
HAS_GCC=0
HAS_MAKE=0

if command -v gcc > /dev/null 2>&1; then HAS_GCC=1; fi
if command -v make > /dev/null 2>&1; then HAS_MAKE=1; fi

if [ "$HAS_GCC" -eq 0 ] || [ "$HAS_MAKE" -eq 0 ]; then
    printf "${YELLOW}⚠️ ERROR: No se detectaron las herramientas de compilación (gcc/make).${NC}\n"
    printf "Rust necesita 'build-essential' para compilar el sistema.\n\n"
    
    if command -v apt > /dev/null 2>&1; then
        printf "Ejecuta: ${GREEN}sudo apt update && sudo apt install build-essential -y${NC}\n"
    elif command -v dnf > /dev/null 2>&1; then
        printf "Ejecuta: ${GREEN}sudo dnf groupinstall \"Development Tools\"${NC}\n"
    elif command -v pacman > /dev/null 2>&1; then
        printf "Ejecuta: ${GREEN}sudo pacman -S base-devel${NC}\n"
    else
        printf "Por favor, instala las herramientas de desarrollo de tu distribución.\n"
    fi
    exit 1
fi

# 2. Clonar repositorio
TARGET_DIR="$HOME/Documents/inventario_pro"
if [ -d "$TARGET_DIR" ]; then
    printf "Actualizando repositorio existente en $TARGET_DIR...\n"
    cd "$TARGET_DIR" && git pull
else
    printf "Clonando repositorio...\n"
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- "$TARGET_DIR"
    cd "$TARGET_DIR"
fi

# 3. Compilar
printf "${CYAN}Compilando aplicación (esto puede tardar unos minutos)...${NC}\n"
cargo build --release

if [ $? -eq 0 ]; then
    printf "${GREEN}✅ ¡Instalación completada con éxito!${NC}\n"
    printf "Para iniciar el sistema, ejecuta:\n"
    printf "${YELLOW}cd $TARGET_DIR && cargo run --release${NC}\n"
else
    printf "${RED}❌ Hubo un error durante la compilación.${NC}\n"
fi
