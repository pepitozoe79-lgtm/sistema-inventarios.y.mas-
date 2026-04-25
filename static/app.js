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
                    <p>${usuario.username} <span class="badge plan-badge plan-${usuario.plan.toLowerCase()}">${usuario.plan}</span></p>
                </div>
                <ul>
                    <li onclick="cargarDashboard()">📊 Dashboard</li>
                    ${usuario.rol === 'superadmin' ? '<li onclick="cargarSuperAdmin()" style="color:#38bdf8;">🛡️ SuperAdmin</li>' : ''}
                    <li onclick="cargarAnalytics()">📈 Analítica BI</li>
                    <li onclick="cargarPredictivo()">🔮 Predicciones</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarVentas()">💰 Ventas</li>
                    <li onclick="cargarSettings()">⚙️ Integraciones</li>
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- ⚙️ SETTINGS, API KEYS & WEBHOOKS ---
async function cargarSettings() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando configuración de desarrollador...</h2>';
    try {
        const [keysRes, webhooksRes, logsRes] = await Promise.all([
            apiFetch('/settings/api-keys'),
            apiFetch('/settings/webhooks'),
            apiFetch('/settings/webhooks/logs')
        ]);
        
        main.innerHTML = `
            <h2>⚙️ Configuración para Desarrolladores</h2>
            
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-top:2rem;">
                <section>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                        <h3>🔑 API Keys</h3>
                        <button class="btn-primary" style="width:auto;" onclick="abrirModalNuevaKey()">+ Nueva</button>
                    </div>
                    ${keysRes.data.map(k => `
                        <div class="api-key-card">
                            <div><strong>${k.nombre}</strong><br>${k.scopes.split(',').map(s => `<span class="scope-pill">${s}</span>`).join('')}</div>
                            <button class="btn-danger" style="width:auto; padding:0.4rem;" onclick="eliminarKey('${k.id}')">Revocar</button>
                        </div>
                    `).join('')}
                </section>

                <section>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                        <h3>📡 Webhooks</h3>
                        <button class="btn-primary" style="width:auto;" onclick="abrirModalNuevoWebhook()">+ Registrar</button>
                    </div>
                    ${webhooksRes.data.map(w => `
                        <div class="webhook-card">
                            <div style="word-break:break-all;"><strong>URL:</strong> ${w.url}</div>
                            <div style="margin-top:0.5rem;"><strong>Eventos:</strong> ${w.event_types.split(',').map(e => `<span class="event-type-badge">${e}</span>`).join('')}</div>
                            <div style="font-size:0.7rem; margin-top:0.5rem; color:#64748b;">Secret: <code>${w.secret}</code></div>
                            <button class="btn-danger" style="width:auto; margin-top:1rem;" onclick="eliminarWebhook('${w.id}')">Eliminar Endpoint</button>
                        </div>
                    `).join('')}
                </section>
            </div>

            <section style="margin-top:3rem;">
                <h3>🕒 Registro de Entregas (Logs)</h3>
                <div class="card" style="padding:0;">
                    ${logsRes.data.map(l => `
                        <div class="log-row">
                            <div><span class="status-dot ${l.status_code >= 200 && l.status_code < 300 ? 'status-success' : 'status-error'}"></span> <strong>${l.event_type}</strong></div>
                            <div style="color:#64748b;">${l.status_code || 'TIMED_OUT'}</div>
                            <div style="font-size:0.7rem;">${l.fecha.substring(11,19)}</div>
                        </div>
                    `).join('')}
                </div>
            </section>
        `;
    } catch (e) {}
}

async function abrirModalNuevoWebhook() {
    const url = prompt("URL de destino del Webhook:");
    if (!url) return;
    try {
        await apiFetch('/settings/webhooks', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ url, event_types: ["sale.created", "stock.low"] })
        });
        cargarSettings();
    } catch (e) {}
}

async function eliminarWebhook(id) {
    if (!confirm("¿Eliminar este endpoint?")) return;
    try {
        await apiFetch(`/settings/webhooks/${id}`, { method: 'DELETE' });
        cargarSettings();
    } catch (e) {}
}

// ... Resto de funciones (API Keys, Dashboard, etc.) ...
async function eliminarKey(id) { /* ... */ }
async function abrirModalNuevaKey() { /* ... */ }
async function generarNuevaKey(nombre) { /* ... */ }

async function cargarDashboard() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = res.data;
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h2>📊 Resumen de Operaciones</h2>
                <span class="badge badge-info">${usuario.plan}</span>
            </div>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #6366f1;"><h3>Ventas</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #ef4444;"><h3>Gastos</h3><p>$${stats.gastos_hoy.toFixed(2)}</p></div>
            </div>
        `;
    } catch (e) {}
}

// Funciones vacías para evitar errores de referencia
function cargarAnalytics() {}
function cargarPredictivo() {}
function cargarPOS() {}
function cargarProductos() {}
function cargarVentas() {}
function upgradeToPro() {}

if (token) mostrarDashboardUI(); else mostrarLogin();
