#!/bin/bash

echo "--- Desinstalador de Sistema de Inventario ---"
INSTALL_DIR="$HOME/inventario_sys"

if [ -d "$INSTALL_DIR" ]; then
    read -p "¿Estás seguro de que deseas eliminar el sistema y todos sus datos? (s/n): " confirm
    if [ "$confirm" == "s" ]; then
        rm -rf "$INSTALL_DIR"
        echo "Sistema eliminado correctamente."
    else
        echo "Operación cancelada."
    fi
else
    echo "No se encontró la instalación en $INSTALL_DIR"
fi
