#!/bin/sh

# Colores (compatibles con printf)
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

printf "${CYAN}--- Instalador Universal: Inventario Pro (Linux) ---${NC}\n"

# 1. Verificar herramientas de compilación
if ! command -v gcc > /dev/null 2>&1 || ! command -v make > /dev/null 2>&1; then
    printf "${YELLOW}⚠️ ERROR: No se detectaron las herramientas de compilación (gcc/make).${NC}\n"
    printf "Ejecuta: ${GREEN}sudo apt update && sudo apt install build-essential -y${NC}\n"
    exit 1
fi

# 2. Verificar si Rust está instalado
if ! command -v cargo > /dev/null 2>&1; then
    printf "${YELLOW}⚠️ ERROR: No se detectó Rust (cargo).${NC}\n"
    printf "Por favor, instala Rust ejecutando:\n"
    printf "${GREEN}curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}\n"
    printf "Luego reinicia tu terminal e intenta de nuevo.\n"
    exit 1
fi

# 3. Clonar repositorio
TARGET_DIR="$HOME/Documents/inventario_pro"
if [ -d "$TARGET_DIR" ]; then
    printf "Actualizando repositorio existente en $TARGET_DIR...\n"
    cd "$TARGET_DIR" && git pull
else
    printf "Clonando repositorio...\n"
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- "$TARGET_DIR"
    cd "$TARGET_DIR"
fi

# 4. Compilar
printf "${CYAN}Compilando aplicación (esto puede tardar unos minutos)...${NC}\n"
cargo build --release

if [ $? -eq 0 ]; then
    printf "${GREEN}✅ ¡Instalación completada con éxito!${NC}\n"
    printf "Para iniciar el sistema, ejecuta:\n"
    printf "${YELLOW}cd $TARGET_DIR && cargo run --release${NC}\n"
else
    printf "${RED}❌ Hubo un error durante la compilación.${NC}\n"
fi
