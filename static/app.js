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
    if (response.status === 401) { cerrarSesion(); throw new Error("Sesión expirada"); }
    const result = await response.json();
    if (!response.ok) {
        const errorMsg = result.error || "Error desconocido";
        const errorCode = result.code || "UNKNOWN_ERROR";
        alert(`Error (${errorCode}): ${errorMsg}`);
        throw { message: errorMsg, code: errorCode };
    }
    return result;
}

// --- Autenticación ---
async function login(e) {
    e.preventDefault();
    try {
        const result = await apiFetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username: e.target.username.value, password: e.target.password.value })
        });
        token = result.data.token; usuario = result.data.usuario;
        localStorage.setItem('token', token); localStorage.setItem('usuario', JSON.stringify(usuario));
        mostrarDashboardUI();
    } catch (e) {}
}

function cerrarSesion() {
    token = null; usuario = null;
    localStorage.removeItem('token'); localStorage.removeItem('usuario');
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
                <div class="sidebar-header"><h3>Inventario Pro</h3><p>${usuario.username}</p></div>
                <ul>
                    <li onclick="cargarDashboard()">📊 Dashboard</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarMovimientos()">🚛 Kardex</li>
                    <li onclick="cargarVentas()">💰 Ventas</li>
                    <li onclick="cargarGastos()">💸 Gastos</li>
                    ${usuario.rol === 'admin' ? '<li onclick="cargarUsuarios()">👥 Usuarios</li>' : ''}
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- 📊 DASHBOARD (P&L Inteligente) ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Analizando finanzas...</h2>';
    try {
        const result = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = result.data;
        main.innerHTML = `
            <h2>📊 Inteligencia de Negocio</h2>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #2ecc71;">
                    <h3>Ventas Hoy</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p>
                </div>
                <div class="kpi-card" style="border-left-color: #e74c3c;">
                    <h3>Gastos Hoy</h3><p>$${stats.gastos_hoy.toFixed(2)}</p>
                </div>
                <div class="kpi-card" style="border-left-color: #f1c40f;">
                    <h3>Utilidad Neta</h3>
                    <p class="${stats.utilidad_hoy >= 0 ? 'text-success' : 'text-danger'}">
                        $${stats.utilidad_hoy.toFixed(2)}
                    </p>
                </div>
                <div class="kpi-card" style="border-left-color: #3498db;">
                    <h3>Stock Alertas</h3><p>${stats.alertas_stock_bajo}</p>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <div class="card">
                    <h3>🏆 Top Productos</h3>
                    ${top_productos.map(p => `<div style="margin-bottom:0.5rem;">${p.nombre}: <strong>${p.cantidad}</strong></div>`).join('')}
                </div>
                <div class="card">
                    <h3>🕒 Actividad Reciente</h3>
                    ${actividad.map(a => `
                        <div style="font-size:0.8rem; padding:0.5rem 0; border-bottom:1px solid #eee;">
                            <span class="badge ${a.tipo === 'VENTA' ? 'badge-success' : a.tipo === 'GASTO' ? 'badge-danger' : 'badge-info'}">${a.tipo}</span> 
                            ${a.descripcion}
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
    } catch (e) {}
}

// --- 💸 GASTOS (Egresos) ---
async function cargarGastos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/gastos');
        const gastos = result.data;
        main.innerHTML = `
            <div class="header-actions">
                <h2>💸 Control de Gastos</h2>
                ${usuario.rol === 'admin' ? '<button class="btn-primary" onclick="mostrarFormGasto()">+ Registrar Gasto</button>' : ''}
            </div>
            <div id="form-gasto" class="card" style="display:none; margin-bottom: 2rem;">
                <h3>Registrar Egreso</h3>
                <form onsubmit="crearGasto(event)">
                    <select name="tipo" required>
                        <option value="OPERATIVO">OPERATIVO (Luz, Agua, etc)</option>
                        <option value="MERCADERIA">MERCADERIA (Compras)</option>
                        <option value="SUELDOS">SUELDOS</option>
                        <option value="OTROS">OTROS</option>
                    </select>
                    <input type="number" step="0.01" name="monto" placeholder="Monto $" required style="margin-top:0.5rem;">
                    <input type="text" name="descripcion" placeholder="Descripción" style="margin-top:0.5rem;">
                    <button type="submit" class="btn-primary" style="margin-top:0.5rem;">Guardar Gasto</button>
                </form>
            </div>
            <table class="card">
                <thead><tr><th>Fecha</th><th>Tipo</th><th>Descripción</th><th>Monto</th><th>Acciones</th></tr></thead>
                <tbody>
                    ${gastos.map(g => `
                        <tr>
                            <td>${g.fecha.substring(0,10)}</td>
                            <td><span class="badge badge-warning">${g.tipo}</span></td>
                            <td>${g.descripcion || '-'}</td>
                            <td class="text-danger">-$${g.monto.toFixed(2)}</td>
                            <td>${usuario.rol === 'admin' ? `<button onclick="eliminarGasto('${g.id}')">🗑️</button>` : '-'}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {}
}

function mostrarFormGasto() { const f = document.getElementById('form-gasto'); f.style.display = f.style.display === 'none' ? 'block' : 'none'; }

async function crearGasto(e) {
    e.preventDefault();
    try {
        await apiFetch('/gastos', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tipo: e.target.tipo.value, monto: parseFloat(e.target.monto.value), descripcion: e.target.descripcion.value })
        });
        cargarGastos();
    } catch (e) {}
}

async function eliminarGasto(id) {
    if (!confirm("¿Eliminar gasto?")) return;
    try { await apiFetch(`/gastos/${id}`, { method: 'DELETE' }); cargarGastos(); } catch (e) {}
}

// --- 🛒 POS ---
async function cargarPOS() {
    const main = document.getElementById('main-content'); carrito = [];
    try {
        const result = await apiFetch('/productos'); productosCache = result.data;
        main.innerHTML = `
            <div class="pos-container">
                <div class="pos-column card"><div id="pos-product-list">${renderPOSProducts(productosCache)}</div></div>
                <div class="pos-column card"><h3>🛒 Carrito</h3><div id="pos-cart-list"></div></div>
                <div class="pos-column"><div class="checkout-panel"><h1>TOTAL</h1><h2 id="pos-total">$0.00</h2><button class="btn-secondary" onclick="finalizarVenta()">VENTA</button></div></div>
            </div>
        `;
    } catch (e) {}
}

function renderPOSProducts(list) { return list.map(p => `<div class="product-item-pos" onclick="agregarAlCarrito('${p.id}')"><strong>${p.nombre}</strong><br>$${p.precio_unitario}</div>`).join(''); }

function agregarAlCarrito(id) {
    const p = productosCache.find(x => x.id === id); const item = carrito.find(x => x.id === id);
    if (item) { item.cantidad++; } else { carrito.push({ ...p, cantidad: 1 }); }
    actualizarVistaCarrito();
}

function actualizarVistaCarrito() {
    const list = document.getElementById('pos-cart-list'); const totalEl = document.getElementById('pos-total');
    let total = 0;
    list.innerHTML = carrito.map(item => { total += item.cantidad * item.precio_unitario; return `<div>${item.nombre} x ${item.cantidad}</div>`; }).join('');
    totalEl.innerText = `$${total.toFixed(2)}`;
}

async function finalizarVenta() {
    try {
        const res = await apiFetch('/ventas', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ lineas: carrito.map(i => ({ producto_id: i.id, cantidad: i.cantidad })) }) });
        const ventaId = res.data.venta.id; if (confirm("¿Factura?")) { descargarFactura(ventaId); }
        cargarDashboard();
    } catch (e) {}
}

// --- 📄 FACTURACIÓN ---
async function descargarFactura(ventaId) {
    try {
        const res = await fetch(`${API_BASE}/ventas/${ventaId}/factura`, { headers: { 'Authorization': `Bearer ${token}` } });
        const blob = await res.blob(); const url = window.URL.createObjectURL(blob); const a = document.createElement('a');
        a.href = url; a.download = `factura_${ventaId.substring(0,8)}.pdf`; a.click();
    } catch (e) {}
}

// Otros módulos simplificados para ahorro de espacio
async function cargarProductos() {
    const main = document.getElementById('main-content');
    const res = await apiFetch('/productos');
    main.innerHTML = `<h2>📦 Productos</h2><table class="card"><tbody>${res.data.map(p => `<tr><td>${p.nombre}</td><td>${p.stock_actual}</td></tr>`).join('')}</tbody></table>`;
}

async function cargarMovimientos() {
    const main = document.getElementById('main-content');
    const res = await apiFetch('/inventario/movimientos');
    main.innerHTML = `<h2>🚛 Kardex</h2><table class="card"><tbody>${res.data.map(m => `<tr><td>${m.tipo}</td><td>${m.stock_antes} -> ${m.stock_despues}</td></tr>`).join('')}</tbody></table>`;
}

async function cargarVentas() {
    const main = document.getElementById('main-content');
    const res = await apiFetch('/ventas');
    main.innerHTML = `<h2>💰 Ventas</h2><table class="card"><tbody>${res.data.map(v => `<tr><td>${v.fecha}</td><td>$${v.total}</td></tr>`).join('')}</tbody></table>`;
}

async function cargarUsuarios() {
    const main = document.getElementById('main-content');
    const res = await apiFetch('/usuarios');
    main.innerHTML = `<h2>👥 Usuarios</h2><table class="card"><tbody>${res.data.map(u => `<tr><td>${u.username}</td><td>${u.rol}</td></tr>`).join('')}</tbody></table>`;
}

if (token) mostrarDashboardUI(); else mostrarLogin();
function filtrarPOS() {}
function mostrarFormGasto() { const f = document.getElementById('form-gasto'); f.style.display = f.style.display === 'none' ? 'block' : 'none'; }
