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
                    <li onclick="cargarMarketplace()">🛒 Marketplace</li>
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

// --- 🛒 MARKETPLACE ---
async function cargarMarketplace() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando Apps del Marketplace...</h2>';
    try {
        const apps = await apiFetch('/ecosistema/marketplace');
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                <h2>🛒 Marketplace de Aplicaciones</h2>
                <p>Expande las capacidades de tu ERP con integraciones oficiales.</p>
            </div>
            <div class="marketplace-grid">
                ${apps.map(a => `
                    <div class="app-card">
                        <div>
                            <div class="app-icon">${a.app.id === 'shopify_sync' ? '🛍️' : a.app.id === 'whatsapp_notify' ? '📱' : '📊'}</div>
                            <h3>${a.app.nombre} ${a.app.premium ? '<span class="premium-tag">PRO</span>' : ''}</h3>
                            <p style="font-size:0.85rem; color:#64748b; margin: 0.5rem 0;">${a.app.descripcion}</p>
                            <div style="font-size:0.75rem; margin-top:1rem;">
                                <strong>Eventos:</strong> ${a.app.eventos_requeridos.split(',').map(e => `<span class="event-type-badge">${e}</span>`).join('')}
                            </div>
                        </div>
                        <div style="margin-top:2rem;">
                            ${a.instalada 
                                ? `<button class="btn-secondary" onclick="desinstalarApp('${a.app.id}')">Desinstalar</button>` 
                                : `<button class="btn-primary" onclick="instalarApp('${a.app.id}')">Instalar</button>`
                            }
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    } catch (e) {}
}

async function instalarApp(appId) {
    const configStr = prompt("Introduce los parámetros de configuración (JSON):", "{}");
    if (configStr === null) return;
    try {
        const configJson = JSON.parse(configStr);
        await apiFetch('/ecosistema/marketplace/instalar', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ app_id: appId, config_json: configJson })
        });
        alert("Aplicación instalada correctamente.");
        cargarMarketplace();
    } catch (e) { alert("Error en el formato JSON"); }
}

async function desinstalarApp(appId) {
    if (!confirm("¿Desinstalar esta aplicación?")) return;
    try {
        await apiFetch(`/ecosistema/marketplace/${appId}`, { method: 'DELETE' });
        cargarMarketplace();
    } catch (e) {}
}

// --- ⚙️ SETTINGS & WEBHOOKS ---
async function cargarSettings() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando...</h2>';
    try {
        const [keysRes, webhooksRes, logsRes] = await Promise.all([
            apiFetch('/settings/api-keys'),
            apiFetch('/settings/webhooks'),
            apiFetch('/settings/webhooks/logs')
        ]);
        main.innerHTML = `
            <h2>⚙️ Configuración de Integración</h2>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; margin-top:2rem;">
                <section>
                    <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:1rem;">
                        <h3>🔑 API Keys</h3>
                        <button class="btn-primary" style="width:auto;" onclick="abrirModalNuevaKey()">+ Nueva</button>
                    </div>
                    ${keysRes.data.map(k => `<div class="api-key-card"><div><strong>${k.nombre}</strong></div><button class="btn-danger" style="width:auto;" onclick="eliminarKey('${k.id}')">Revocar</button></div>`).join('')}
                </section>
                <section>
                    <h3>📡 Webhooks</h3>
                    ${webhooksRes.data.map(w => `<div class="webhook-card"><div><strong>URL:</strong> ${w.url}</div><button class="btn-danger" style="width:auto;" onclick="eliminarWebhook('${w.id}')">Eliminar</button></div>`).join('')}
                </section>
            </div>
        `;
    } catch (e) {}
}

// Otros módulos simplificados...
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/dashboard');
        const { stats } = res.data;
        main.innerHTML = `
            <div style="display:flex; justify-content:space-between; align-items:center;">
                <h2>📊 Dashboard</h2>
                <span class="badge badge-info">${usuario.plan}</span>
            </div>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #6366f1;"><h3>Ventas Hoy</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
            </div>
        `;
    } catch (e) {}
}

function cargarAnalytics() {}
function cargarPredictivo() {}
function cargarPOS() {}
function cargarProductos() {}
function cargarVentas() {}
function upgradeToPro() {}
function cerrarSesion() { localStorage.clear(); location.reload(); }

if (token) mostrarDashboardUI(); else mostrarLogin();
