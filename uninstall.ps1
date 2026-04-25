Write-Host "--- Desinstalador de Sistema de Inventario ---" -ForegroundColor Red
$installDir = "$HOME\Documents\inventario_sys"

if (Test-Path $installDir) {
    $confirm = Read-Host "¿Estás seguro de que deseas eliminar el sistema y todos sus datos? (S/N)"
    if ($confirm -eq "S" -or $confirm -eq "s") {
        Remove-Item -Recurse -Force $installDir
        Write-Host "Sistema eliminado correctamente." -ForegroundColor Green
    } else {
        Write-Host "Operación cancelada."
    }
} else {
    Write-Host "No se encontró la instalación."
}
