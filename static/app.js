let token = localStorage.getItem('token');
let usuario = JSON.parse(localStorage.getItem('usuario'));
let carrito = [];
let productosCache = [];

// --- Utilidades ---
const API_BASE = '/api/v1';

async function apiFetch(endpoint, options = {}) {
    if (!options.headers) options.headers = {};
    if (token) options.headers['Authorization'] = `Bearer ${token}`;
    
    const response = await fetch(`${API_BASE}${endpoint}`, options);
    
    if (response.status === 401) {
        cerrarSesion();
        throw new Error("Sesión expirada");
    }

    const result = await response.json();

    if (!response.ok) {
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

    return result;
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
        mostrarDashboardUI();
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

function mostrarDashboardUI() {
    app.innerHTML = `
        <div class="dashboard">
            <nav class="sidebar">
                <div class="sidebar-header">
                    <h3>Inventario Pro</h3>
                    <p>${usuario.username}</p>
                </div>
                <ul>
                    <li onclick="cargarDashboard()">📊 Dashboard</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta (POS)</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarMovimientos()">🚛 Kardex / Bodega</li>
                    <li onclick="cargarVentas()">💰 Historial Ventas</li>
                    ${usuario.rol === 'admin' ? '<li onclick="cargarUsuarios()">👥 Usuarios</li>' : ''}
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content">
                <!-- Se carga vía cargarDashboard() -->
            </main>
        </div>
    `;
    cargarDashboard();
}

// --- 📊 DASHBOARD (INTELIGENCIA) ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Analizando datos de negocio...</h2>';
    
    try {
        const result = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = result.data;

        const maxQty = Math.max(...top_productos.map(p => p.cantidad), 1);

        main.innerHTML = `
            <h2>📊 Dashboard de Inteligencia</h2>
            
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #2ecc71;">
                    <h3>Ventas Hoy (Ingresos)</h3>
                    <p>$${stats.ventas_hoy_total.toFixed(2)}</p>
                </div>
                <div class="kpi-card" style="border-left-color: #3498db;">
                    <h3>Tickets Emitidos</h3>
                    <p>${stats.ventas_hoy_cantidad}</p>
                </div>
                <div class="kpi-card" style="border-left-color: #9b59b6;">
                    <h3>Unidades Vendidas</h3>
                    <p>${stats.productos_vendidos_hoy}</p>
                </div>
                <div class="kpi-card" style="border-left-color: #e74c3c;">
                    <h3>Alertas de Stock</h3>
                    <p>${stats.alertas_stock_bajo}</p>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <!-- Gráfico de Top Productos -->
                <div class="card">
                    <h3>🏆 Top 5 Productos más Vendidos</h3>
                    <div class="bar-chart-container">
                        ${top_productos.map(p => {
                            const width = (p.cantidad / maxQty) * 100;
                            return `
                                <div class="bar-row">
                                    <div class="bar-label">${p.nombre}</div>
                                    <div class="bar-track">
                                        <div class="bar-fill" style="width: ${width}%"></div>
                                    </div>
                                    <div class="bar-value">${p.cantidad} und.</div>
                                </div>
                            `;
                        }).join('')}
                        ${top_productos.length === 0 ? '<p style="text-align:center; color:#999;">Sin ventas registradas</p>' : ''}
                    </div>
                </div>

                <!-- Feed de Actividad -->
                <div class="card">
                    <h3>🕒 Actividad Reciente</h3>
                    <ul style="list-style: none; padding: 0;">
                        ${actividad.map(a => `
                            <li style="padding: 0.8rem 0; border-bottom: 1px solid #eee; display: flex; justify-content: space-between; align-items: center;">
                                <div>
                                    <span class="badge ${a.tipo === 'VENTA' ? 'badge-success' : 'badge-info'}" style="font-size: 0.7rem;">${a.tipo}</span>
                                    <span style="margin-left: 0.5rem; font-size: 0.9rem;">${a.descripcion}</span>
                                </div>
                                <small style="color: #999;">${a.fecha.substring(11, 16)}</small>
                            </li>
                        `).join('')}
                    </ul>
                </div>
            </div>
        `;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar inteligencia de negocio</h2>`;
    }
}

// --- 🛒 PUNTO DE VENTA (POS) ---
async function cargarPOS() {
    const main = document.getElementById('main-content');
    carrito = [];
    
    try {
        const result = await apiFetch('/productos');
        productosCache = result.data;

        main.innerHTML = `
            <div class="pos-container">
                <div class="pos-column card">
                    <div style="padding: 1rem; border-bottom: 1px solid #eee;">
                        <input type="text" id="pos-search" placeholder="Buscar producto..." oninput="filtrarPOS(this.value)">
                    </div>
                    <div id="pos-product-list">${renderPOSProducts(productosCache)}</div>
                </div>
                <div class="pos-column card">
                    <h3 style="padding: 1rem; border-bottom: 1px solid #eee; margin:0;">🛒 Carrito</h3>
                    <div id="pos-cart-list" style="flex-grow: 1;"><p style="text-align:center; padding: 2rem; color: #999;">Vacío</p></div>
                </div>
                <div class="pos-column">
                    <div class="checkout-panel">
                        <p style="margin:0; opacity: 0.8;">TOTAL</p>
                        <h1 id="pos-total" style="margin: 0.5rem 0;">$0.00</h1>
                        <button class="btn-secondary" style="width: 100%;" onclick="finalizarVenta()">CONFIRMAR VENTA</button>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {}
}

function renderPOSProducts(list) {
    return list.map(p => `
        <div class="product-item-pos" onclick="agregarAlCarrito('${p.id}')">
            <div style="display: flex; justify-content: space-between;"><strong>${p.nombre}</strong><span>$${p.precio_unitario.toFixed(2)}</span></div>
            <div style="font-size: 0.8rem; color: #666;">Stock: <span class="${p.stock_actual <= 5 ? 'text-danger' : ''}">${p.stock_actual}</span></div>
        </div>
    `).join('');
}

function filtrarPOS(query) {
    const filtered = productosCache.filter(p => p.nombre.toLowerCase().includes(query.toLowerCase()) || p.codigo.toLowerCase().includes(query.toLowerCase()));
    document.getElementById('pos-product-list').innerHTML = renderPOSProducts(filtered);
}

function agregarAlCarrito(id) {
    const p = productosCache.find(x => x.id === id);
    const item = carrito.find(x => x.id === id);
    if (item) {
        if (item.cantidad + 1 > p.stock_actual) return alert("Stock insuficiente");
        item.cantidad++;
    } else {
        if (p.stock_actual < 1) return alert("Sin stock");
        carrito.push({ ...p, cantidad: 1 });
    }
    actualizarVistaCarrito();
}

function actualizarCantidad(id, delta) {
    const item = carrito.find(x => x.id === id);
    const p = productosCache.find(x => x.id === id);
    if (item.cantidad + delta <= 0) carrito = carrito.filter(x => x.id !== id);
    else if (item.cantidad + delta > p.stock_actual) return alert("Máximo stock");
    else item.cantidad += delta;
    actualizarVistaCarrito();
}

function actualizarVistaCarrito() {
    const list = document.getElementById('pos-cart-list');
    const totalEl = document.getElementById('pos-total');
    if (carrito.length === 0) {
        list.innerHTML = `<p style="text-align:center; padding: 2rem; color: #999;">Vacío</p>`;
        totalEl.innerText = '$0.00';
        return;
    }
    let total = 0;
    list.innerHTML = carrito.map(item => {
        total += item.cantidad * item.precio_unitario;
        return `<div class="cart-item"><div>${item.nombre}</div><div><button onclick="actualizarCantidad('${item.id}', -1)">-</button> ${item.cantidad} <button onclick="actualizarCantidad('${item.id}', 1)">+</button></div></div>`;
    }).join('');
    totalEl.innerText = `$${total.toFixed(2)}`;
}

async function finalizarVenta() {
    if (carrito.length === 0) return;
    try {
        await apiFetch('/ventas', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ lineas: carrito.map(i => ({ producto_id: i.id, cantidad: i.cantidad })) })
        });
        alert("💰 Venta Exitosa");
        cargarDashboard(); // Volver al dashboard después de vender
    } catch (e) {}
}

// 📦 PRODUCTOS
async function cargarProductos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/productos');
        const productos = result.data;
        main.innerHTML = `
            <div class="header-actions"><h2>📦 Productos</h2><button class="btn-primary" onclick="mostrarFormProducto()">+ Nuevo</button></div>
            <div id="form-producto" class="card" style="display:none; margin-bottom: 2rem;">
                <h3>Nuevo Producto</h3>
                <form onsubmit="crearProducto(event)">
                    <input type="text" name="codigo" placeholder="Código" required>
                    <input type="text" name="nombre" placeholder="Nombre" required>
                    <input type="number" step="0.01" name="precio" placeholder="Precio" required>
                    <button type="submit" class="btn-primary">Guardar</button>
                </form>
            </div>
            <table class="card">
                <thead><tr><th>Código</th><th>Nombre</th><th>Precio</th><th>Stock</th></tr></thead>
                <tbody>${productos.map(p => `<tr><td>${p.codigo}</td><td>${p.nombre}</td><td>$${p.precio_unitario}</td><td>${p.stock_actual}</td></tr>`).join('')}</tbody>
            </table>
        `;
    } catch (e) {}
}

async function crearProducto(e) {
    e.preventDefault();
    try {
        await apiFetch('/productos', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ codigo: e.target.codigo.value, nombre: e.target.nombre.value, precio_unitario: parseFloat(e.target.precio.value) })
        });
        cargarProductos();
    } catch (e) {}
}

function mostrarFormProducto() {
    const f = document.getElementById('form-producto');
    f.style.display = f.style.display === 'none' ? 'block' : 'none';
}

// 🚛 KARDEX
async function cargarMovimientos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/inventario/movimientos');
        const movimientos = result.data;
        main.innerHTML = `
            <h2>🚛 Kardex de Bodega</h2>
            <table class="card">
                <thead><tr><th>Fecha</th><th>Tipo</th><th>Cant</th><th>Antes</th><th>Después</th></tr></thead>
                <tbody>${movimientos.map(m => `<tr><td>${m.fecha.substring(11, 16)}</td><td>${m.tipo}</td><td>${m.cantidad}</td><td>${m.stock_antes}</td><td>${m.stock_despues}</td></tr>`).join('')}</tbody>
            </table>
        `;
    } catch (e) {}
}

// 💰 VENTAS
async function cargarVentas() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/ventas');
        const ventas = result.data;
        main.innerHTML = `<h2>💰 Historial de Ventas</h2><table class="card"><thead><tr><th>Fecha</th><th>Total</th></tr></thead><tbody>${ventas.map(v => `<tr><td>${v.fecha}</td><td>$${v.total.toFixed(2)}</td></tr>`).join('')}</tbody></table>`;
    } catch (e) {}
}

// 👥 USUARIOS
async function cargarUsuarios() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/usuarios');
        const usuariosList = result.data;
        main.innerHTML = `<h2>👥 Usuarios</h2><table class="card"><thead><tr><th>Usuario</th><th>Rol</th></tr></thead><tbody>${usuariosList.map(u => `<tr><td>${u.username}</td><td>${u.rol}</td></tr>`).join('')}</tbody></table>`;
    } catch (e) {}
}

// Inicialización
if (token) {
    mostrarDashboardUI();
} else {
    mostrarLogin();
}
function mostrarFormMovimiento() { /* No usado en esta simplificación visual */ }
