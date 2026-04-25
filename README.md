# 🛒 Sistema de Inventario, Bodega y Ventas (Rust)

Este es un sistema integral de gestión de inventario, bodega y ventas multiusuario, diseñado para ser ligero, rápido y multiplataforma (Windows, Linux, macOS). Construido con **Rust** en el backend y una interfaz moderna en **Vanilla JavaScript**.

## 🚀 Características principales

- **Gestión Multiusuario**: Autenticación segura mediante JWT y contraseñas hasheadas con Argon2.
- **Control de Roles (RBAC)**: Diferenciación entre administradores y usuarios normales.
- **Gestión de Inventario**: CRUD completo de productos y registro de movimientos (entradas/salidas).
- **Módulo de Ventas**: Creación de ventas con transacciones atómicas, descuento automático de stock y registro de historial.
- **Reportes PDF Profesionales**:
  - Exportación de inventario con filtros (ej: solo productos con stock bajo).
  - Reportes de ventas mensuales con cálculo de totales.
- **Interfaz Moderna**: Dashboard responsive con estética "premium", diseñado para una experiencia de usuario fluida.
- **Base de Datos Embebida**: Utiliza SQLite mediante SQLx para una instalación sin dependencias externas.

## 🛠️ Requisitos previos

- [Rust](https://www.rust-lang.org/tools/install) (Edición 2021).
- Fuentes TrueType (TTF) para los reportes (se recomienda **Roboto**).

## 📦 Instalación y Configuración

1. **Clonar el repositorio** (o descargar los archivos).
2. **Configurar el entorno**: Crea o edita el archivo `.env` en la raíz:
   ```env
   DATABASE_URL=sqlite:inventario.db?mode=rwc
   JWT_SECRET=tu_clave_secreta_aqui
   ```
3. **Instalar Fuentes**:
   - Crea una carpeta llamada `fonts` en la raíz.
   - Coloca los archivos `Roboto-Regular.ttf` y `Roboto-Bold.ttf` dentro.
4. **Ejecutar la aplicación**:
   ```bash
   cargo run
   ```
   La aplicación estará disponible en [http://localhost:3000](http://localhost:3000).

## 📖 Guía de Uso

### Inicio de Sesión y Roles
- El primer usuario que se registre puede ser promovido a **admin** directamente en la base de datos o a través de otro administrador.
- Solo los **Administradores** pueden crear productos, gestionar usuarios y ver reportes avanzados.

### Gestión de Productos
- Accede a la pestaña "Productos" para añadir nuevos ítems.
- Puedes editar precios, descripciones y códigos en cualquier momento.

### Realizar Ventas
- En la pestaña "Ventas", selecciona uno o varios productos, ajusta las cantidades y finaliza la transacción. El sistema validará automáticamente si hay stock suficiente.

### Reportes
- Usa los botones de exportación para obtener documentos PDF listos para imprimir o enviar por correo.

## 🏗️ Arquitectura Técnica

- **Backend**: [Axum](https://github.com/tokio-rs/axum) (Web Framework) + [Tokio](https://tokio.rs/) (Async Runtime).
- **Base de Datos**: [SQLx](https://github.com/launchbadge/sqlx) con SQLite.
- **Seguridad**: JWT (jsonwebtoken) + Argon2 (password hashing).
- **Reportes**: [genpdf](https://github.com/p-avital/genpdf).
- **Frontend**: HTML5, CSS3 (Variables y Flexbox/Grid), Vanilla JS.

---
Desarrollado con ❤️ en Rust.
