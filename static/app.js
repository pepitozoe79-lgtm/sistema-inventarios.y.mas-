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
    if (!response.ok) { alert(`Error: ${result.error || "Desconocido"}`); throw result; }
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
    app.innerHTML = `<div class="login-container"><div class="card"><h1>📦 Inventario Pro SaaS</h1><form onsubmit="login(event)"><input type="text" name="username" placeholder="Usuario" required><input type="password" name="password" placeholder="Contraseña" required><button type="submit" class="btn-primary">Entrar</button></form></div></div>`;
}

function mostrarDashboardUI() {
    app.innerHTML = `
        <div class="dashboard">
            <nav class="sidebar">
                <div class="sidebar-header">
                    <h3>Inventario Pro</h3>
                    <p>${usuario.username} <span class="plan-badge plan-${usuario.plan ? usuario.plan.toLowerCase() : 'basic'}">${usuario.plan || 'BASIC'}</span></p>
                </div>
                <ul>
                    <li onclick="cargarDashboard()">📊 Dashboard</li>
                    ${usuario.rol === 'superadmin' ? '<li onclick="cargarSuperAdmin()" style="color:#38bdf8; font-weight:bold;">🛡️ SuperAdmin</li>' : ''}
                    <li onclick="cargarAnalytics()">📈 Analítica BI</li>
                    <li onclick="cargarPredictivo()">🔮 Predicciones</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarVentas()">💰 Ventas</li>
                    <li onclick="cargarSettings()">⚙️ Integraciones</li>
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
                ${usuario.plan !== 'PRO' && usuario.rol !== 'superadmin' ? `<div class="upgrade-banner" onclick="upgradeToPro()">⭐ Mejora a PRO</div>` : ''}
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- ⚙️ SETTINGS & API KEYS ---
async function cargarSettings() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando integraciones...</h2>';
    try {
        const res = await apiFetch('/settings/api-keys');
        const keys = res.data;
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:2rem;">
                <h2>⚙️ Integraciones y API Keys</h2>
                <button class="btn-primary" style="width:auto;" onclick="abrirModalNuevaKey()">+ Nueva API Key</button>
            </div>
            <div class="card">
                <h3>Tus llaves de acceso</h3>
                <p>Usa estas llaves para conectar aplicaciones externas. <strong>No las compartas.</strong></p>
                <div id="keys-list" style="margin-top:1.5rem;">
                    ${keys.length === 0 ? '<p>No tienes llaves generadas.</p>' : keys.map(k => `
                        <div class="api-key-card">
                            <div>
                                <strong>${k.nombre}</strong>
                                <div style="margin-top:0.4rem;">
                                    ${k.scopes.split(',').map(s => `<span class="scope-pill">${s}</span>`).join('')}
                                </div>
                                <small style="color:#64748b; display:block; margin-top:0.5rem;">Creada el: ${k.creado_en.substring(0,10)}</small>
                            </div>
                            <button class="btn-danger" style="width:auto; padding:0.5rem 1rem;" onclick="eliminarKey('${k.id}')">Revocar</button>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
    } catch (e) {}
}

function abrirModalNuevaKey() {
    const nombre = prompt("Nombre de la integración (ej: Mi E-commerce):");
    if (!nombre) return;
    generarNuevaKey(nombre);
}

async function generarNuevaKey(nombre) {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/settings/api-keys', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ nombre, scopes: ["products:read", "inventory:read"] })
        });
        
        main.innerHTML = `
            <div class="card" style="border: 2px solid #38bdf8;">
                <h2 style="color:#0369a1;">🔑 ¡Llave Generada con Éxito!</h2>
                <p>Copia esta llave ahora. Por seguridad, <strong>no volverá a mostrarse.</strong></p>
                <div class="secret-box">${res.key}</div>
                <p style="font-size:0.8rem; color:#ef4444;">⚠️ Si pierdes esta llave, tendrás que revocarla y crear una nueva.</p>
                <button class="btn-primary" style="width:auto; margin-top:1rem;" onclick="cargarSettings()">He guardado mi llave</button>
            </div>
        `;
    } catch (e) {}
}

async function eliminarKey(id) {
    if (!confirm("¿Estás seguro de revocar esta llave? Las integraciones que la usen dejarán de funcionar.")) return;
    try {
        await apiFetch(`/settings/api-keys/${id}`, { method: 'DELETE' });
        cargarSettings();
    } catch (e) {}
}

// --- 🛡️ SUPERADMIN DASHBOARD ---
async function cargarSuperAdmin() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando inteligencia global...</h2>';
    try {
        const res = await apiFetch('/superadmin/dashboard');
        const { stats, recientes_tenants, pagos_fallidos_recientes } = res.data;
        main.innerHTML = `
            <div class="superadmin-header">
                <h2>🛡️ Panel de Control Global</h2>
                <div class="superadmin-grid">
                    <div class="kpi-global"><h4>MRR</h4><p>$${stats.mrr.toFixed(2)}</p></div>
                    <div class="kpi-global"><h4>ARR</h4><p>$${stats.arr.toFixed(2)}</p></div>
                    <div class="kpi-global"><h4>Churn</h4><p>${stats.churn_rate.toFixed(1)}%</p></div>
                    <div class="kpi-global"><h4>Tenants</h4><p>${stats.tenants_activos}/${stats.total_tenants}</p></div>
                </div>
            </div>
            <div style="display:grid; grid-template-columns: 2fr 1fr; gap: 2rem;">
                <div class="card"><h3>👥 Organizaciones</h3><table class="tenant-table">...</table></div>
            </div>
        `;
    } catch (e) {}
}

// --- 📊 DASHBOARD, POS, etc... ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = res.data;
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h2>📊 Resumen</h2>
                <span class="badge badge-info">${usuario.plan} PLAN</span>
            </div>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #6366f1;"><h3>Ventas</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #ef4444;"><h3>Gastos</h3><p>$${stats.gastos_hoy.toFixed(2)}</p></div>
            </div>
        `;
    } catch (e) {}
}

// ... Resto de funciones (Predictivo, Analytics, etc.)

if (token) mostrarDashboardUI(); else mostrarLogin();
function filtrarPOS() {}
function cargarProductos() {}
function cargarVentas() {}
function upgradeToPro() {}
function cargarPredictivo() {}
function cargarAnalytics() {}
function cargarMovimientos() {}
function cargarGastos() {}
function cargarPOS() {}
