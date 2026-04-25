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
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- 📊 DASHBOARD ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Analizando...</h2>';
    try {
        const result = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = result.data;
        main.innerHTML = `
            <h2>📊 Dashboard</h2>
            <div class="dashboard-grid">
                <div class="kpi-card"><h3>Ingresos Hoy</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
                <div class="kpi-card"><h3>Tickets</h3><p>${stats.ventas_hoy_cantidad}</p></div>
                <div class="kpi-card"><h3>Stock Bajo</h3><p>${stats.alertas_stock_bajo}</p></div>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <div class="card"><h3>🏆 Top Productos</h3>${top_productos.map(p => `<div>${p.nombre}: ${p.cantidad}</div>`).join('')}</div>
                <div class="card"><h3>🕒 Actividad</h3>${actividad.map(a => `<div style="font-size:0.8rem; margin-bottom:0.5rem;">${a.descripcion}</div>`).join('')}</div>
            </div>
        `;
    } catch (e) {}
}

// --- 🛒 POS ---
async function cargarPOS() {
    const main = document.getElementById('main-content');
    carrito = [];
    try {
        const result = await apiFetch('/productos');
        productosCache = result.data;
        main.innerHTML = `
            <div class="pos-container">
                <div class="pos-column card">
                    <input type="text" placeholder="Buscar..." oninput="filtrarPOS(this.value)" style="margin-bottom:1rem;">
                    <div id="pos-product-list">${renderPOSProducts(productosCache)}</div>
                </div>
                <div class="pos-column card">
                    <h3>🛒 Carrito</h3>
                    <div id="pos-cart-list" style="flex-grow:1;"></div>
                </div>
                <div class="pos-column">
                    <div class="checkout-panel">
                        <p>TOTAL</p>
                        <h1 id="pos-total">$0.00</h1>
                        <button class="btn-secondary" style="width:100%;" onclick="finalizarVenta()">FINALIZAR VENTA</button>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {}
}

function renderPOSProducts(list) {
    return list.map(p => `<div class="product-item-pos" onclick="agregarAlCarrito('${p.id}')"><strong>${p.nombre}</strong><br><small>$${p.precio_unitario} | Stock: ${p.stock_actual}</small></div>`).join('');
}

function filtrarPOS(query) {
    const filtered = productosCache.filter(p => p.nombre.toLowerCase().includes(query.toLowerCase()));
    document.getElementById('pos-product-list').innerHTML = renderPOSProducts(filtered);
}

function agregarAlCarrito(id) {
    const p = productosCache.find(x => x.id === id);
    const item = carrito.find(x => x.id === id);
    if (item) {
        if (item.cantidad + 1 > p.stock_actual) return alert("Sin stock");
        item.cantidad++;
    } else {
        if (p.stock_actual < 1) return alert("Sin stock");
        carrito.push({ ...p, cantidad: 1 });
    }
    actualizarVistaCarrito();
}

function actualizarVistaCarrito() {
    const list = document.getElementById('pos-cart-list');
    const totalEl = document.getElementById('pos-total');
    let total = 0;
    list.innerHTML = carrito.map(item => {
        total += item.cantidad * item.precio_unitario;
        return `<div class="cart-item">${item.nombre} x ${item.cantidad}</div>`;
    }).join('');
    totalEl.innerText = `$${total.toFixed(2)}`;
}

async function finalizarVenta() {
    if (carrito.length === 0) return;
    try {
        const result = await apiFetch('/ventas', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ lineas: carrito.map(i => ({ producto_id: i.id, cantidad: i.cantidad })) })
        });
        
        const ventaId = result.data.venta.id;
        if (confirm("💰 Venta exitosa. ¿Deseas descargar la factura?")) {
            descargarFactura(ventaId);
        }
        cargarPOS();
    } catch (e) {}
}

// --- 💰 HISTORIAL VENTAS ---
async function cargarVentas() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/ventas');
        const ventas = result.data;
        main.innerHTML = `
            <h2>💰 Historial de Ventas</h2>
            <table class="card">
                <thead><tr><th>Fecha</th><th>Total</th><th>Acciones</th></tr></thead>
                <tbody>
                    ${ventas.map(v => `
                        <tr>
                            <td>${v.fecha}</td>
                            <td>$${v.total.toFixed(2)}</td>
                            <td><button class="btn-small" onclick="descargarFactura('${v.id}')">📄 Factura PDF</button></td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {}
}

// --- 📄 FACTURACIÓN ---
async function descargarFactura(ventaId) {
    try {
        const response = await fetch(`${API_BASE}/ventas/${ventaId}/factura`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (!response.ok) throw new Error("Error al generar factura");
        
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `factura_${ventaId.substring(0,8)}.pdf`;
        document.body.appendChild(a);
        a.click();
        a.remove();
    } catch (e) {
        alert("No se pudo descargar la factura.");
        console.error(e);
    }
}

// 📦 PRODUCTOS
async function cargarProductos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/productos');
        const productos = result.data;
        main.innerHTML = `
            <div class="header-actions"><h2>📦 Productos</h2><button class="btn-primary" onclick="mostrarFormProducto()">+ Nuevo</button></div>
            <table class="card">
                <thead><tr><th>Código</th><th>Nombre</th><th>Precio</th><th>Stock</th></tr></thead>
                <tbody>${productos.map(p => `<tr><td>${p.codigo}</td><td>${p.nombre}</td><td>$${p.precio_unitario}</td><td>${p.stock_actual}</td></tr>`).join('')}</tbody>
            </table>
        `;
    } catch (e) {}
}

// 🚛 KARDEX
async function cargarMovimientos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/inventario/movimientos');
        const movimientos = result.data;
        main.innerHTML = `<h2>🚛 Kardex</h2><table class="card"><thead><tr><th>Fecha</th><th>Tipo</th><th>Antes</th><th>Después</th></tr></thead><tbody>${movimientos.map(m => `<tr><td>${m.fecha.substring(11, 16)}</td><td>${m.tipo}</td><td>${m.stock_antes}</td><td>${m.stock_despues}</td></tr>`).join('')}</tbody></table>`;
    } catch (e) {}
}

// Inicialización
if (token) mostrarDashboardUI(); else mostrarLogin();
function mostrarFormProducto() {}
function filtrarPOS() {}
