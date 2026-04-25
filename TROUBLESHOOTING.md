# 🛠️ Solución de Problemas (Troubleshooting) - Inventario Pro

Si estás instalando este sistema en Windows y te encuentras con errores, aquí tienes las soluciones rápidas.

---

## ❌ Error: `linker link.exe not found`

### 🔍 ¿Por qué sucede?
Rust en Windows utiliza el motor de compilación de Microsoft (MSVC). Si no tienes instalado el compilador de C++, Rust no puede generar el archivo ejecutable final.

### ✅ Solución definitiva (Recomendada)
1.  Descarga el **Instalador de Visual Studio Build Tools**: [Descargar aquí](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2.  Al ejecutarlo, selecciona la carga de trabajo: **"Desarrollo para el escritorio con C++"** (Desktop development with C++).
3.  Instala y reinicia tu terminal.

### ✅ Solución rápida (Motor GNU)
Si no quieres instalar Visual Studio, puedes cambiar el motor de Rust a GNU:
```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

---

## ❌ Error: `iwr : No se puede analizar el contenido...`

### 🔍 ¿Por qué sucede?
PowerShell intenta usar el motor de Internet Explorer para descargar archivos, y en Windows modernos este motor está desactivado.

### ✅ Solución
Añade siempre el parámetro `-UseBasicParsing` a tus comandos `iwr`:
```powershell
iwr -UseBasicParsing https://raw.githubusercontent.com/... | iex
```

---

## ❌ Error: `Port 3000 is already in use`

### ✅ Solución
1.  Busca el proceso que usa el puerto: `netstat -ano | findstr :3000`
2.  Mata el proceso: `taskkill /F /PID <PID_ENCONTRADO>`
3.  O cambia el puerto en tu archivo `.env`.
