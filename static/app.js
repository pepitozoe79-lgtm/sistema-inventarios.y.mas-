let token = localStorage.getItem('token');
let usuario = JSON.parse(localStorage.getItem('usuario'));
let carrito = [];
let productosCache = [];

// --- Utilidades ---
const API_BASE = '/api/v1';

async function apiFetch(endpoint, options = {}) {
    if (!options.headers) options.headers = {};
    if (token) options.headers['Authorization'] = `Bearer ${token}`;
    const response = await fetch(`${endpoint.startsWith('/api/v2') ? '' : API_BASE}${endpoint}`, options);
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

function cerrarSesion() { localStorage.clear(); location.reload(); }

// --- UI Management ---
const app = document.getElementById('app');

function mostrarLogin() {
    app.innerHTML = `<div class="login-container"><div class="card"><h1>📦 Inventario Pro SaaS</h1><form onsubmit="login(event)"><input type="text" name="username" placeholder="Usuario" required><input type="password" name="password" placeholder="Contraseña" required><button type="submit" class="btn-primary">Entrar</button></form></div></div>`;
}

function mostrarDashboardUI() {
    app.innerHTML = `
        <div class="dashboard">
            <nav class="sidebar">
                <div class="sidebar-header"><h3>Inventario Pro</h3><p>${usuario.username} <span class="badge plan-badge plan-${usuario.plan.toLowerCase()}">${usuario.plan}</span></p></div>
                <ul>
                    <li onclick="cargarDashboard()">📊 Dashboard</li>
                    <li onclick="cargarCopilot()" style="color:#6366f1; font-weight:bold;">🤖 Asistente IA</li>
                    ${usuario.rol === 'superadmin' ? '<li onclick="cargarSuperAdmin()" style="color:#38bdf8;">🛡️ SuperAdmin</li>' : ''}
                    <li onclick="cargarMarketplace()">🛒 Marketplace</li>
                    <li onclick="cargarPOS()">🛒 Punto de Venta</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarSettings()">⚙️ Integraciones</li>
                    <li onclick="cerrarSesion()" class="logout">🚪 Salir</li>
                </ul>
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- 🤖 AI COPILOT ---
async function cargarCopilot() {
    const main = document.getElementById('main-content');
    main.innerHTML = `
        <h2>🤖 Copilot de Negocios</h2>
        <div class="ai-container">
            <div id="chat-box" class="chat-box">
                <div class="chat-msg msg-ai">¡Hola! Soy tu asistente de IA. Puedo ayudarte con el inventario, ventas o enviar alertas. Prueba preguntando: <em>"¿Cuánto vendí hoy?"</em></div>
            </div>
            <div id="suggestions" style="margin-bottom:1rem;">
                <button class="suggestion-btn" onclick="enviarQueryAI('¿Cuánto vendí hoy?')">Ventas de hoy</button>
                <button class="suggestion-btn" onclick="enviarQueryAI('¿Qué productos debo reponer?')">Alerta de stock</button>
            </div>
            <form onsubmit="event.preventDefault(); enviarQueryAI(this.query.value); this.reset();" style="display:flex; gap:1rem;">
                <input type="text" name="query" placeholder="Pregúntame algo sobre tu negocio..." required style="flex-grow:1;">
                <button type="submit" class="btn-primary" style="width:auto;">Enviar</button>
            </form>
        </div>
    `;
}

async function enviarQueryAI(texto) {
    const chatBox = document.getElementById('chat-box');
    chatBox.innerHTML += `<div class="chat-msg msg-user">${texto}</div>`;
    chatBox.scrollTop = chatBox.scrollHeight;

    try {
        const res = await apiFetch('/api/v2/ai/query', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ query: texto })
        });

        const actionsHtml = res.actions_taken.map(a => `<div class="ai-tool-badge">🛠️ ${a.tool}: ${a.status}</div>`).join('');
        
        chatBox.innerHTML += `
            <div class="chat-msg msg-ai">
                ${res.answer}
                <div style="margin-top:0.5rem; border-top: 1px solid #f1f5f9; padding-top:0.5rem;">
                    ${actionsHtml}
                </div>
            </div>
        `;

        // Actualizar sugerencias
        const suggContainer = document.getElementById('suggestions');
        suggContainer.innerHTML = res.suggested_commands.map(s => `
            <button class="suggestion-btn" onclick="enviarQueryAI('${s}')">${s}</button>
        `).join('');

        chatBox.scrollTop = chatBox.scrollHeight;
    } catch (e) {}
}

// Otros módulos simplificados...
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/dashboard');
        main.innerHTML = `<h2>📊 Dashboard</h2><div class="dashboard-grid"><div class="kpi-card"><h3>Ventas</h3><p>$${res.data.stats.ventas_hoy_total.toFixed(2)}</p></div></div>`;
    } catch (e) {}
}

async function cargarMarketplace() {
    const main = document.getElementById('main-content');
    const apps = await apiFetch('/ecosistema/marketplace');
    main.innerHTML = `<h2>🛒 Marketplace</h2><div class="marketplace-grid">${apps.map(a => `<div class="app-card"><h3>${a.app.nombre}</h3><p>${a.app.descripcion}</p></div>`).join('')}</div>`;
}

// ... Resto de funciones (Settings, SuperAdmin, etc.) ...
function cargarSuperAdmin() {}
function cargarPOS() {}
function cargarProductos() {}
function cargarSettings() {}

if (token) mostrarDashboardUI(); else mostrarLogin();
