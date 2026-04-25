Write-Host "--- Instalador Inteligente: Inventario Pro (Windows) ---" -ForegroundColor Cyan

# 1. Verificar herramientas de compilación (Linker)
$linker = Get-Command link.exe -ErrorAction SilentlyContinue
if (-not $linker) {
    Write-Host "⚠️ ERROR CRÍTICO: No se detectaron las herramientas de compilación de C++ (MSVC)." -ForegroundColor Yellow
    Write-Host "Rust necesita el Linker de Microsoft para crear el ejecutable." -ForegroundColor White
    Write-Host ""
    Write-Host "SOLUCIÓN: Por favor, descarga e instala las 'C++ Build Tools' desde aquí:" -ForegroundColor Green
    Write-Host "👉 https://visualstudio.microsoft.com/visual-cpp-build-tools/"
    Write-Host "Selecciona la opción: 'Desarrollo para el escritorio con C++'"
    Write-Host ""
    Write-Host "Una vez instalado, reinicia tu terminal y vuelve a ejecutar este comando."
    exit
}

# 2. Clonar repositorio
$targetDir = "$HOME\Documents\inventario_pro"
if (Test-Path $targetDir) {
    Write-Host "Actualizando repositorio existente..." -ForegroundColor Gray
    cd $targetDir
    git pull
} else {
    Write-Host "Clonando repositorio..." -ForegroundColor Gray
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- $targetDir
    cd $targetDir
}

# 3. Compilar
Write-Host "Compilando aplicación (esto puede tardar unos minutos)..." -ForegroundColor Cyan
cargo build --release

if ($?) {
    Write-Host "✅ ¡Instalación completada con éxito!" -ForegroundColor Green
    Write-Host "Para iniciar el sistema, ejecuta:" -ForegroundColor White
    Write-Host "cd $targetDir; cargo run --release" -ForegroundColor Yellow
} else {
    Write-Host "❌ Hubo un error durante la compilación." -ForegroundColor Red
}
