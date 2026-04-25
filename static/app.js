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
                    <li onclick="cargarAnalytics()">📈 Analítica BI</li>
                    <li onclick="cargarPredictivo()">🔮 Predicciones</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarMovimientos()">🚛 Kardex</li>
                    <li onclick="cargarVentas()">💰 Ventas</li>
                    <li onclick="cargarGastos()">💸 Gastos</li>
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
                ${usuario.plan !== 'PRO' ? `<div class="upgrade-banner" onclick="upgradeToPro()">⭐ Mejora a PRO</div>` : ''}
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- 💳 BILLING ---
async function upgradeToPro() {
    try {
        const res = await apiFetch('/billing/checkout', { method: 'POST' });
        alert(res.message);
        // En un entorno real: window.location.href = res.url;
        // Para demo simulamos éxito inmediato (esto lo haría el webhook en prod)
        console.log("Simulando redirección a:", res.url);
        window.open(res.url, '_blank');
    } catch (e) {}
}

// --- 📊 DASHBOARD ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando...</h2>';
    try {
        const res = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = res.data;
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h2>📊 Resumen de Hoy</h2>
                ${usuario.plan === 'BASIC' ? '<div style="color:#f59e0b; font-size:0.8rem; font-weight:bold;">⚠️ Estás usando el Plan Básico (Límites activos)</div>' : ''}
            </div>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #6366f1;"><h3>Ventas</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #ef4444;"><h3>Gastos</h3><p>$${stats.gastos_hoy.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #22c55e;"><h3>Utilidad</h3><p class="${stats.utilidad_hoy >= 0 ? 'text-success' : 'text-danger'}">$${stats.utilidad_hoy.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #f59e0b;" onclick="cargarPredictivo()"><h3>Alertas</h3><p>${stats.alertas_stock_bajo}</p></div>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <div class="card"><h3>🏆 Top Productos</h3>${top_productos.map(p => `<div>${p.nombre}: ${p.cantidad}</div>`).join('')}</div>
                <div class="card"><h3>🕒 Actividad</h3>${actividad.map(a => `<div style="font-size:0.8rem; margin-bottom:0.4rem;"><span class="badge ${a.tipo === 'VENTA' ? 'badge-success' : a.tipo === 'GASTO' ? 'badge-danger' : 'badge-info'}">${a.tipo}</span> ${a.descripcion}</div>`).join('')}</div>
            </div>
        `;
    } catch (e) {}
}

// --- 🔮 PREDICTIVO (Con Bloqueo) ---
async function cargarPredictivo() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/predictivo');
        const { stock_en_riesgo, forecast_ventas } = res.data;
        main.innerHTML = `
            <h2>🔮 Predicción</h2>
            <div class="forecast-header"><h1>$${forecast_ventas.ventas_proximos_7_dias.toFixed(2)}</h1><p>Proyección 7 días</p></div>
            <div class="card"><h3>⚠️ Riesgo Stock</h3>${stock_en_riesgo.map(p => `<div class="risk-card risk-${p.riesgo.toLowerCase()}"><div>${p.nombre}</div><div class="countdown-timer">${Math.floor(p.dias_restantes)} <small>días</small></div></div>`).join('')}</div>
        `;
    } catch (err) {
        if (err.code === "FORBIDDEN") {
            main.innerHTML = `
                <div class="card" style="text-align:center; padding:4rem;">
                    <h1 style="font-size:4rem;">🔒</h1>
                    <h2>Módulo Predictivo Bloqueado</h2>
                    <p>La inteligencia predictiva solo está disponible para usuarios PRO.</p>
                    <button class="btn-primary" style="width:auto; margin-top:1rem;" onclick="upgradeToPro()">Mejorar a PRO ahora</button>
                </div>
            `;
        }
    }
}

// Otros módulos simplificados...
async function cargarAnalytics() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/analytics');
        main.innerHTML = `<h2>📈 Analítica</h2><div class="card">Análisis habilitado para tu plan PRO.</div>`;
    } catch (err) {
        if (err.code === "FORBIDDEN") {
            main.innerHTML = `<div class="card" style="text-align:center; padding:4rem;"><h1>🔒</h1><h2>Analítica Bloqueada</h2><button class="btn-primary" onclick="upgradeToPro()">Mejorar a PRO</button></div>`;
        }
    }
}

async function cargarPOS() { /* ... */ }
async function cargarProductos() { /* ... */ }
async function cargarMovimientos() { /* ... */ }
async function cargarVentas() { /* ... */ }
async function cargarGastos() { /* ... */ }

if (token) mostrarDashboardUI(); else mostrarLogin();
function filtrarPOS() {}
