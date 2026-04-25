# 🚀 Guía de Despliegue - Sistema de Inventario Rust

Este documento detalla cómo poner en funcionamiento el sistema en un entorno de producción para cada sistema operativo.

## 🪟 Despliegue en Windows

### 1. Compilación
Abre PowerShell en la carpeta del proyecto y ejecuta:
```powershell
cargo build --release
```
El ejecutable se generará en `target\release\inventario_sys.exe`.

### 2. Estructura de Producción
Crea una carpeta de despliegue (ej. `C:\InventarioApp`) y copia solo lo necesario:
- `inventario_sys.exe` (desde `target\release\`)
- Carpeta `static\` (completa)
- Carpeta `fonts\` (con las fuentes Roboto)
- Archivo `.env` (configura tu `JWT_SECRET` real)

### 3. Ejecución como Servicio (Opcional)
Para que el sistema inicie siempre con Windows, puedes usar **NSSM** o el **Programador de Tareas**:
- Crea una tarea que ejecute `inventario_sys.exe`.
- Asegúrate de marcar "Iniciar en" con la ruta de la carpeta donde está el ejecutable para que encuentre los archivos estáticos.

---

## 🐧 Despliegue en Linux (Ubuntu/Debian/CentOS)

### 1. Compilación
```bash
cargo build --release
```
El binario estará en `target/release/inventario_sys`.

### 2. Configuración de Systemd (Servicio)
Crea un archivo de servicio para que el sistema se mantenga activo:
`sudo nano /etc/systemd/system/inventario.service`

```ini
[Unit]
Description=Sistema de Inventario Rust
After=network.target

[Service]
User=tu_usuario
WorkingDirectory=/var/www/inventario
ExecStart=/var/www/inventario/inventario_sys
Restart=always
Environment=DATABASE_URL=sqlite:/var/www/inventario/inventario.db?mode=rwc
Environment=JWT_SECRET=tu_clave_secreta

[Install]
WantedBy=multi-user.target
```

### 3. Iniciar el servicio
```bash
sudo systemctl daemon-reload
sudo systemctl enable inventario
sudo systemctl start inventario
```

---

## 🍎 Despliegue en macOS

### 1. Compilación
```bash
cargo build --release
```
El binario estará en `target/release/inventario_sys`.

### 2. Estructura y Permisos
Mueve el binario y las carpetas `static` y `fonts` a una ubicación segura (ej. `/Applications/InventarioApp`). Asegúrate de dar permisos de ejecución:
```bash
chmod +x inventario_sys
```

### 3. Ejecución automática (Launchd)
Crea un archivo `.plist` en `~/Library/LaunchAgents/com.inventario.sys.plist`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.inventario.sys</string>
    <key>ProgramArguments</key>
    <array>
        <string>/ruta/a/tu/app/inventario_sys</string>
    </array>
    <key>WorkingDirectory</key>
    <string>/ruta/a/tu/app</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
</dict>
</plist>
```
Luego cárgalo: `launchctl load ~/Library/LaunchAgents/com.inventario.sys.plist`

---

## 🌐 Notas Generales de Red
- El sistema corre por defecto en el puerto **3000**.
- Para acceso desde internet, abre el puerto en tu firewall:
  - **Windows**: `New-NetFirewallRule -DisplayName "Inventario" -Direction Inbound -LocalPort 3000 -Protocol TCP -Action Allow`
  - **Linux (UFW)**: `sudo ufw allow 3000/tcp`
- Se recomienda usar un proxy inverso como **Nginx** o **Caddy** para añadir soporte HTTPS (SSL).
