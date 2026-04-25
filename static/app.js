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
    app.innerHTML = `<div class="login-container"><div class="card"><h1>📦 Inventario Pro</h1><form onsubmit="login(event)"><input type="text" name="username" placeholder="Usuario" required><input type="password" name="password" placeholder="Contraseña" required><button type="submit" class="btn-primary">Entrar</button></form></div></div>`;
}

function mostrarDashboardUI() {
    app.innerHTML = `
        <div class="dashboard">
            <nav class="sidebar">
                <div class="sidebar-header"><h3>Inventario Pro</h3><p>${usuario.username}</p></div>
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
            </nav>
            <main class="content" id="main-content"></main>
        </div>
    `;
    cargarDashboard();
}

// --- 📊 DASHBOARD ---
async function cargarDashboard() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Cargando...</h2>';
    try {
        const res = await apiFetch('/dashboard');
        const { stats, top_productos, actividad } = res.data;
        main.innerHTML = `
            <h2>📊 Resumen de Hoy</h2>
            <div class="dashboard-grid">
                <div class="kpi-card" style="border-left-color: #6366f1;"><h3>Ventas</h3><p>$${stats.ventas_hoy_total.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #ef4444;"><h3>Gastos</h3><p>$${stats.gastos_hoy.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #22c55e;"><h3>Utilidad</h3><p class="${stats.utilidad_hoy >= 0 ? 'text-success' : 'text-danger'}">$${stats.utilidad_hoy.toFixed(2)}</p></div>
                <div class="kpi-card" style="border-left-color: #f59e0b;" onclick="cargarPredictivo()" style="cursor:pointer;"><h3>Alertas</h3><p>${stats.alertas_stock_bajo}</p></div>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <div class="card"><h3>🏆 Top Productos</h3>${top_productos.map(p => `<div>${p.nombre}: ${p.cantidad}</div>`).join('')}</div>
                <div class="card"><h3>🕒 Actividad</h3>${actividad.map(a => `<div style="font-size:0.8rem; margin-bottom:0.4rem;"><span class="badge ${a.tipo === 'VENTA' ? 'badge-success' : a.tipo === 'GASTO' ? 'badge-danger' : 'badge-info'}">${a.tipo}</span> ${a.descripcion}</div>`).join('')}</div>
            </div>
        `;
    } catch (e) {}
}

// --- 🔮 PREDICTIVO ---
async function cargarPredictivo() {
    const main = document.getElementById('main-content');
    main.innerHTML = '<h2>Calculando proyecciones futuras...</h2>';
    try {
        const res = await apiFetch('/predictivo');
        const { stock_en_riesgo, forecast_ventas } = res.data;

        main.innerHTML = `
            <h2>🔮 Motor de Predicción Operativa</h2>
            
            <div class="forecast-header">
                <p style="margin:0; opacity:0.9; font-size:1.1rem;">Ventas Proyectadas (Próximos 7 días)</p>
                <h1 style="margin:0.5rem 0; font-size:3.5rem;">$${forecast_ventas.ventas_proximos_7_dias.toFixed(2)}</h1>
                <div style="display:inline-block; padding:0.5rem 1rem; background:rgba(255,255,255,0.2); border-radius:30px;">
                    Tendencia: <strong>${forecast_ventas.tendencia}</strong> | Confianza: ${ (forecast_ventas.confianza * 100).toFixed(0) }%
                </div>
            </div>

            <div style="display:grid; grid-template-columns: 1fr 1fr; gap: 2rem;">
                <!-- Riesgo de Inventario -->
                <div class="card">
                    <h3>⚠️ Productos en Riesgo de Quiebre</h3>
                    <p style="color:#666; font-size:0.9rem; margin-bottom:1.5rem;">Días estimados antes de agotar stock basado en velocidad de venta real.</p>
                    ${stock_en_riesgo.map(p => `
                        <div class="risk-card risk-${p.riesgo.toLowerCase()}">
                            <div>
                                <strong>${p.nombre}</strong><br>
                                <small>Stock: ${p.stock_actual} | Vel: ${p.velocidad_diaria.toFixed(2)} und/día</small>
                            </div>
                            <div class="countdown-timer">
                                ${p.dias_restantes >= 100 ? '∞' : Math.floor(p.dias_restantes)} <small>días</small>
                            </div>
                        </div>
                    `).join('')}
                    ${stock_en_riesgo.length === 0 ? '<p style="text-align:center; padding:2rem; color:#999;">No hay riesgos detectados.</p>' : ''}
                </div>

                <!-- Análisis Predictivo -->
                <div class="card">
                    <h3>💡 Insights del Sistema</h3>
                    <div style="padding:1rem; background:#f8fafc; border-radius:8px; border-left:4px solid #6366f1; margin-bottom:1rem;">
                        <p style="margin:0;"><strong>Sugerencia de Compra:</strong> Basado en el forecast, deberías reabastecer los productos con riesgo 🔴 hoy mismo para evitar pérdida de ventas el fin de semana.</p>
                    </div>
                    <div style="padding:1rem; background:#f8fafc; border-radius:8px; border-left:4px solid #3498db;">
                        <p style="margin:0;"><strong>Flujo de Caja:</strong> La tendencia <strong>${forecast_ventas.tendencia}</strong> indica que tendrás liquidez suficiente para cubrir los gastos operativos proyectados.</p>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {}
}

// --- 📈 ANALÍTICA (BI) ---
async function cargarAnalytics() {
    const main = document.getElementById('main-content');
    try {
        const res = await apiFetch('/analytics');
        const { serie_30_dias, comparativa, ticket_promedio } = res.data;
        const maxVal = Math.max(...serie_30_dias.map(d => Math.max(d.ventas, d.gastos)), 1);
        main.innerHTML = `
            <h2>📈 Inteligencia BI</h2>
            <div class="dashboard-grid">
                <div class="kpi-card"><h3>Ticket Promedio</h3><p>$${ticket_promedio.toFixed(2)}</p></div>
                <div class="kpi-card"><h3>Crecimiento</h3><span class="growth-badge ${comparativa.crecimiento_porcentaje >= 0 ? 'growth-up' : 'growth-down'}">${comparativa.crecimiento_porcentaje.toFixed(1)}%</span></div>
            </div>
            <div class="card">
                <h3>Histórico 30 días</h3>
                <div class="analytics-chart">
                    ${serie_30_dias.map(d => `<div class="chart-bar-group" data-date="${d.fecha}"><div class="bar-sales" style="height:${(d.ventas/maxVal)*100}%"></div><div class="bar-expenses" style="height:${(d.gastos/maxVal)*100}%"></div></div>`).join('')}
                </div>
            </div>
        `;
    } catch (e) {}
}

// Otros módulos simplificados para ahorrar espacio
async function cargarPOS() { /* Implementación POS */ }
async function cargarProductos() { /* Implementación Productos */ }
async function cargarMovimientos() { /* Implementación Kardex */ }
async function cargarVentas() { /* Implementación Ventas */ }
async function cargarGastos() { /* Implementación Gastos */ }

if (token) mostrarDashboardUI(); else mostrarLogin();
function filtrarPOS() {}
