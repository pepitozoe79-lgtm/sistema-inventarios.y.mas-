Write-Host "--- Instalador de Sistema de Inventario (Windows) ---" -ForegroundColor Blue

# 1. Verificar Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Rust no detectado. Instalando rustup..." -ForegroundColor Yellow
    $url = "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe"
    Invoke-WebRequest -Uri $url -OutFile "rustup-init.exe"
    Start-Process -FilePath ".\rustup-init.exe" -ArgumentList "-y" -Wait
    Remove-Item ".\rustup-init.exe"
    $env:Path += ";$HOME\.cargo\bin"
}

# 2. Clonar repo
$installDir = "$HOME\Documents\inventario_sys"
if (!(Test-Path $installDir)) {
    Write-Host "Clonando repositorio..." -ForegroundColor Yellow
    git clone https://github.com/pepitozoe79-lgtm/sistema-inventarios.y.mas- $installDir
}
Set-Location $installDir

# 3. Fuentes
if (!(Test-Path "fonts")) { New-Item -ItemType Directory -Name "fonts" }

# 4. .env
if (!(Test-Path ".env")) {
    Write-Host "Creando archivo .env..."
    "DATABASE_URL=sqlite:inventario.db?mode=rwc" | Out-File -FilePath .env -Encoding utf8
    "JWT_SECRET=$( [Convert]::ToBase64String((1..32 | % { [byte](Get-Random -Minimum 0 -Maximum 255) })) )" | Add-Content -Path .env
}

# 5. Compilar
Write-Host "Compilando aplicación..." -ForegroundColor Green
cargo build --release

Write-Host "Instalación completada." -ForegroundColor Green
Write-Host "Para iniciar, entra en la carpeta y ejecuta: cargo run --release" -ForegroundColor Blue
