let token = localStorage.getItem('token');
let usuario = JSON.parse(localStorage.getItem('usuario'));

// --- Utilidades ---
const API_BASE = '/api/v1';

async fn apiFetch(endpoint, options = {}) {
    if (!options.headers) options.headers = {};
    if (token) options.headers['Authorization'] = `Bearer ${token}`;
    
    const response = await fetch(`${API_BASE}${endpoint}`, options);
    
    if (response.status === 401) {
        cerrarSesion();
        throw new Error("Sesión expirada");
    }

    const result = await response.json();

    if (!response.ok) {
        // Manejo de errores estandarizado (v1)
        const errorMsg = result.error || "Error desconocido";
        const errorCode = result.code || "UNKNOWN_ERROR";
        
        switch (errorCode) {
            case "CONFLICT":
                alert(`⚠️ Conflicto: ${errorMsg}`);
                break;
            case "VALIDATION_ERROR":
                alert(`❌ Datos inválidos: ${errorMsg}`);
                break;
            case "NOT_FOUND":
                alert(`🔍 No encontrado: ${errorMsg}`);
                break;
            case "FORBIDDEN":
                alert("🚫 No tienes permisos para realizar esta acción.");
                break;
            default:
                alert(`Error (${errorCode}): ${errorMsg}`);
        }
        throw { message: errorMsg, code: errorCode };
    }

    return result; // Devuelve { data: ..., meta: ... } o { data: ... }
}

// --- Autenticación ---
async function login(e) {
    e.preventDefault();
    const username = e.target.username.value;
    const password = e.target.password.value;

    try {
        const result = await apiFetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        });
        
        token = result.data.token;
        usuario = result.data.usuario;
        localStorage.setItem('token', token);
        localStorage.setItem('usuario', JSON.stringify(usuario));
        mostrarDashboard();
    } catch (e) {
        console.error("Login failed", e);
    }
}

async function registro(e) {
    e.preventDefault();
    const username = e.target.username.value;
    const password = e.target.password.value;

    try {
        await apiFetch('/registro', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password })
        });
        alert("Registro exitoso. Ahora puedes iniciar sesión.");
        mostrarLogin();
    } catch (e) {
        console.error("Registro failed", e);
    }
}

function cerrarSesion() {
    token = null;
    usuario = null;
    localStorage.removeItem('token');
    localStorage.removeItem('usuario');
    mostrarLogin();
}

// --- UI Management ---
const app = document.getElementById('app');

function mostrarLogin() {
    app.innerHTML = `
        <div class="login-container">
            <div class="card">
                <h1>📦 Inventario Pro v1</h1>
                <form onsubmit="login(event)">
                    <input type="text" name="username" placeholder="Usuario" required>
                    <input type="password" name="password" placeholder="Contraseña" required>
                    <button type="submit" class="btn-primary">Entrar</button>
                </form>
                <p>¿No tienes cuenta? <a href="#" onclick="mostrarRegistro()">Regístrate</a></p>
            </div>
        </div>
    `;
}

function mostrarRegistro() {
    app.innerHTML = `
        <div class="login-container">
            <div class="card">
                <h1>Crear Cuenta</h1>
                <form onsubmit="registro(event)">
                    <input type="text" name="username" placeholder="Usuario" required>
                    <input type="password" name="password" placeholder="Contraseña" required>
                    <button type="submit" class="btn-primary">Registrar</button>
                </form>
                <p><a href="#" onclick="mostrarLogin()">Volver al login</a></p>
            </div>
        </div>
    `;
}

function mostrarDashboard() {
    app.innerHTML = `
        <div class="dashboard">
            <nav class="sidebar">
                <div class="sidebar-header">
                    <h3>Inventario Pro</h3>
                    <p>${usuario.username} (${usuario.rol})</p>
                </div>
                <ul>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarMovimientos()">🚛 Bodega</li>
                    <li onclick="cargarVentas()">💰 Ventas</li>
                    ${usuario.rol === 'admin' ? '<li onclick="cargarUsuarios()">👥 Usuarios</li>' : ''}
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content">
                <h1>Bienvenido al Sistema</h1>
                <p>Selecciona una opción del menú para comenzar.</p>
            </main>
        </div>
    `;
    cargarProductos();
}

// --- Módulos ---

// 📦 PRODUCTOS
async function cargarProductos() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando productos...</h2>';
    
    try {
        const result = await apiFetch('/productos');
        const productos = result.data;
        const total = result.meta?.total || 0;

        let html = `
            <div class="header-actions">
                <h2>📦 Productos (${total})</h2>
                <div class="botones-header">
                    <label style="font-size: 0.8rem; color: var(--secondary); margin-right: 1rem;">
                        <input type="checkbox" id="check-stock-bajo" style="width: auto;"> Solo stock bajo (≤5)
                    </label>
                    <button class="btn-primary" onclick="mostrarFormProducto()">+ Nuevo producto</button>
                    <button class="btn-secondary" onclick="descargarReporteInventario()">📄 Exportar PDF</button>
                </div>
            </div>
            <div id="form-producto" class="card" style="display:none; margin-bottom: 2rem;">
                <h3>Nuevo Producto</h3>
                <form onsubmit="crearProducto(event)">
                    <input type="text" name="codigo" placeholder="Código" required>
                    <input type="text" name="nombre" placeholder="Nombre" required>
                    <input type="text" name="descripcion" placeholder="Descripción">
                    <input type="number" step="0.01" name="precio" placeholder="Precio Unitario" required>
                    <button type="submit" class="btn-primary">Guardar</button>
                </form>
            </div>
            <table class="card">
                <thead>
                    <tr>
                        <th>Código</th>
                        <th>Nombre</th>
                        <th>Precio</th>
                        <th>Stock</th>
                        <th>Acciones</th>
                    </tr>
                </thead>
                <tbody>
                    ${productos.map(p => `
                        <tr>
                            <td>${p.codigo}</td>
                            <td>${p.nombre}</td>
                            <td>$${p.precio_unitario.toFixed(2)}</td>
                            <td><span class="badge ${p.stock_actual <= 5 ? 'badge-danger' : 'badge-success'}">${p.stock_actual}</span></td>
                            <td>
                                <button class="btn-small" onclick="eliminarProducto('${p.id}')">🗑️</button>
                            </td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
        main.innerHTML = html;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar productos</h2><p>${e.message}</p>`;
    }
}

function mostrarFormProducto() {
    const form = document.getElementById('form-producto');
    form.style.display = form.style.display === 'none' ? 'block' : 'none';
}

async function crearProducto(e) {
    e.preventDefault();
    const dto = {
        codigo: e.target.codigo.value,
        nombre: e.target.nombre.value,
        descripcion: e.target.descripcion.value,
        precio_unitario: parseFloat(e.target.precio.value)
    };

    try {
        await apiFetch('/productos', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(dto)
        });
        cargarProductos();
    } catch (e) {
        console.error("Error al crear producto", e);
    }
}

async function eliminarProducto(id) {
    if (!confirm('¿Seguro que deseas eliminar este producto?')) return;
    try {
        await apiFetch(`/productos/${id}`, { method: 'DELETE' });
        cargarProductos();
    } catch (e) {
        console.error("Error al eliminar", e);
    }
}

// 🚛 MOVIMIENTOS (BODEGA)
async function cargarMovimientos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/inventario/movimientos');
        const movimientos = result.data;

        main.innerHTML = `
            <div class="header-actions">
                <h2>🚛 Movimientos de Bodega</h2>
                <button class="btn-primary" onclick="mostrarFormMovimiento()">+ Registrar Movimiento</button>
            </div>
            <div id="form-movimiento" class="card" style="display:none; margin-bottom: 2rem;">
                <h3>Registrar Entrada/Salida</h3>
                <form onsubmit="registrarMovimiento(event)">
                    <input type="text" name="producto_id" placeholder="ID del Producto" required>
                    <select name="tipo">
                        <option value="entrada">Entrada (+)</option>
                        <option value="salida">Salida (-)</option>
                    </select>
                    <input type="number" name="cantidad" placeholder="Cantidad" required>
                    <input type="text" name="motivo" placeholder="Motivo (ej: Compra, Ajuste)">
                    <button type="submit" class="btn-primary">Registrar</button>
                </form>
            </div>
            <table class="card">
                <thead>
                    <tr>
                        <th>Fecha</th>
                        <th>Producto</th>
                        <th>Tipo</th>
                        <th>Cant.</th>
                        <th>Motivo</th>
                    </tr>
                </thead>
                <tbody>
                    ${movimientos.map(m => `
                        <tr>
                            <td>${m.fecha}</td>
                            <td>${m.producto_id}</td>
                            <td><span class="badge ${m.tipo === 'entrada' ? 'badge-success' : 'badge-danger'}">${m.tipo.toUpperCase()}</span></td>
                            <td>${m.cantidad}</td>
                            <td>${m.motivo || '-'}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar bodega</h2>`;
    }
}

function mostrarFormMovimiento() {
    const form = document.getElementById('form-movimiento');
    form.style.display = form.style.display === 'none' ? 'block' : 'none';
}

async function registrarMovimiento(e) {
    e.preventDefault();
    const endpoint = e.target.tipo.value === 'entrada' ? '/inventario/entrada' : '/inventario/salida';
    const body = {
        producto_id: e.target.producto_id.value,
        cantidad: parseInt(e.target.cantidad.value),
        motivo: e.target.motivo.value
    };

    try {
        await apiFetch(endpoint, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(body)
        });
        cargarMovimientos();
    } catch (e) {
        console.error("Error en movimiento", e);
    }
}

// 💰 VENTAS
async function cargarVentas() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/ventas');
        const ventas = result.data;

        main.innerHTML = `
            <div class="header-actions">
                <h2>💰 Ventas</h2>
                <div class="botones-header" style="display: flex; gap: 0.5rem; align-items: center;">
                    <input type="month" id="reporte-mes" value="${new Date().toISOString().slice(0, 7)}" style="width: auto; margin: 0;">
                    <button class="btn-secondary" onclick="descargarReporteVentas()" style="width: auto; margin: 0;">📊 Reporte Mensual PDF</button>
                    <button class="btn-primary" onclick="mostrarFormVenta()">+ Nueva venta</button>
                </div>
            </div>
            <div id="form-venta" class="card" style="display:none; margin-bottom: 2rem;">
                <h3>Nueva venta</h3>
                <form onsubmit="crearVenta(event)">
                    <p>Formato: ID_PRODUCTO:CANTIDAD (uno por línea)</p>
                    <textarea name="items" placeholder="ej: uuid-producto:2" required></textarea>
                    <button type="submit" class="btn-primary">Finalizar Venta</button>
                </form>
            </div>
            <table class="card">
                <thead>
                    <tr>
                        <th>ID Venta</th>
                        <th>Fecha</th>
                        <th>Total</th>
                    </tr>
                </thead>
                <tbody>
                    ${ventas.map(v => `
                        <tr>
                            <td>${v.id.substring(0,8)}...</td>
                            <td>${v.fecha}</td>
                            <td>$${v.total.toFixed(2)}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar ventas</h2>`;
    }
}

function mostrarFormVenta() {
    const form = document.getElementById('form-venta');
    form.style.display = form.style.display === 'none' ? 'block' : 'none';
}

async function crearVenta(e) {
    e.preventDefault();
    const lines = e.target.items.value.trim().split('\n');
    const lineas = lines.map(l => {
        const [id, cant] = l.split(':');
        return { producto_id: id.trim(), cantidad: parseInt(cant.trim()) };
    });

    try {
        await apiFetch('/ventas', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ lineas })
        });
        cargarVentas();
    } catch (e) {
        console.error("Error al crear venta", e);
    }
}

// 👥 USUARIOS (Solo Admin)
async function cargarUsuarios() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/usuarios');
        const usuariosList = result.data;

        main.innerHTML = `
            <h2>👥 Gestión de Usuarios</h2>
            <table class="card">
                <thead>
                    <tr>
                        <th>Usuario</th>
                        <th>Rol Actual</th>
                        <th>Cambiar Rol</th>
                    </tr>
                </thead>
                <tbody>
                    ${usuariosList.map(u => `
                        <tr>
                            <td>${u.username}</td>
                            <td><span class="badge">${u.rol}</span></td>
                            <td>
                                <select onchange="actualizarRol('${u.id}', this.value)">
                                    <option value="usuario" ${u.rol === 'usuario' ? 'selected' : ''}>Usuario</option>
                                    <option value="admin" ${u.rol === 'admin' ? 'selected' : ''}>Admin</option>
                                </select>
                            </td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar usuarios</h2>`;
    }
}

async function actualizarRol(id, nuevoRol) {
    try {
        await apiFetch(`/usuarios/${id}/rol`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ rol: nuevoRol })
        });
        alert("Rol actualizado correctamente");
    } catch (e) {
        console.error("Error al actualizar rol", e);
    }
}

// ---------- Reportes ----------
async function descargarReporteInventario() {
    const soloStockBajo = document.getElementById('check-stock-bajo')?.checked;
    const urlParams = soloStockBajo ? '?solo_stock_bajo=true&stock_minimo=5' : '';
    
    try {
        const response = await fetch(`${API_BASE}/reportes/inventario${urlParams}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (!response.ok) throw new Error(await response.text());
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `reporte_inventario${soloStockBajo ? '_stock_bajo' : ''}.pdf`;
        document.body.appendChild(a);
        a.click();
        a.remove();
    } catch (e) {
        alert('Error al generar PDF.');
        console.error(e);
    }
}

async function descargarReporteVentas() {
    const inputMes = document.getElementById('reporte-mes').value;
    if (!inputMes) return alert('Selecciona un mes');
    const [anio, mes] = inputMes.split('-');
    
    try {
        const response = await fetch(`${API_BASE}/reportes/ventas?anio=${anio}&mes=${mes}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (!response.ok) throw new Error(await response.text());
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `reporte_ventas_${anio}_${mes}.pdf`;
        document.body.appendChild(a);
        a.click();
        a.remove();
    } catch (e) {
        alert('Error al generar reporte de ventas.');
        console.error(e);
    }
}

// Inicialización
if (token) {
    mostrarDashboard();
} else {
    mostrarLogin();
}
