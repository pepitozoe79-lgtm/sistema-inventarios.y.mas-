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
                    <p>${usuario.username}</p>
                </div>
                <ul>
                    <li onclick="cargarPOS()">🛒 Punto de Venta (POS)</li>
                    <li onclick="cargarProductos()">📦 Productos</li>
                    <li onclick="cargarMovimientos()">🚛 Kardex / Bodega</li>
                    <li onclick="cargarVentas()">💰 Historial Ventas</li>
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
    cargarPOS();
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
                <!-- Columna 1: Catálogo -->
                <div class="pos-column card">
                    <div style="padding: 1rem; border-bottom: 1px solid #eee;">
                        <input type="text" id="pos-search" placeholder="Buscar producto (nombre o código)..." oninput="filtrarPOS(this.value)">
                    </div>
                    <div id="pos-product-list">
                        ${renderPOSProducts(productosCache)}
                    </div>
                </div>

                <!-- Columna 2: Carrito -->
                <div class="pos-column card">
                    <h3 style="padding: 1rem; border-bottom: 1px solid #eee; margin:0;">🛒 Carrito</h3>
                    <div id="pos-cart-list" style="flex-grow: 1;">
                        <p style="text-align:center; padding: 2rem; color: #999;">El carrito está vacío</p>
                    </div>
                </div>

                <!-- Columna 3: Checkout -->
                <div class="pos-column">
                    <div class="checkout-panel">
                        <p style="margin:0; font-size: 0.9rem; opacity: 0.8;">TOTAL A PAGAR</p>
                        <h1 id="pos-total" style="margin: 0.5rem 0; font-size: 2.5rem;">$0.00</h1>
                        <button class="btn-secondary" style="width: 100%; font-size: 1.2rem; padding: 1rem; margin-top: 1rem;" onclick="finalizarVenta()">
                            CONFIRMAR VENTA
                        </button>
                        <button class="btn-small" style="margin-top: 1rem; opacity: 0.7; color: white; background: transparent; border: 1px solid white;" onclick="vaciarCarrito()">
                            Vaciar Carrito
                        </button>
                    </div>
                    <div class="card" style="margin-top: 1rem; padding: 1rem;">
                        <h4>Detalles</h4>
                        <p id="pos-items-count">Productos: 0</p>
                    </div>
                </div>
            </div>
        `;
    } catch (e) {
        main.innerHTML = `<h2>Error al cargar el POS</h2>`;
    }
}

function renderPOSProducts(list) {
    return list.map(p => `
        <div class="product-item-pos" onclick="agregarAlCarrito('${p.id}')">
            <div style="display: flex; justify-content: space-between;">
                <strong>${p.nombre}</strong>
                <span>$${p.precio_unitario.toFixed(2)}</span>
            </div>
            <div style="font-size: 0.8rem; color: #666; margin-top: 0.3rem;">
                Código: ${p.codigo} | Stock: <span class="${p.stock_actual <= 5 ? 'text-danger' : ''}">${p.stock_actual}</span>
            </div>
        </div>
    `).join('');
}

function filtrarPOS(query) {
    const filtered = productosCache.filter(p => 
        p.nombre.toLowerCase().includes(query.toLowerCase()) || 
        p.codigo.toLowerCase().includes(query.toLowerCase())
    );
    document.getElementById('pos-product-list').innerHTML = renderPOSProducts(filtered);
}

function agregarAlCarrito(id) {
    const producto = productosCache.find(p => p.id === id);
    const enCarrito = carrito.find(item => item.id === id);

    if (enCarrito) {
        if (enCarrito.cantidad + 1 > producto.stock_actual) {
            alert(`⚠️ Stock insuficiente para ${producto.nombre}`);
            return;
        }
        enCarrito.cantidad++;
    } else {
        if (producto.stock_actual < 1) {
            alert(`⚠️ No hay stock disponible para ${producto.nombre}`);
            return;
        }
        carrito.push({ ...producto, cantidad: 1 });
    }
    actualizarVistaCarrito();
}

function actualizarCantidad(id, delta) {
    const item = carrito.find(i => i.id === id);
    const original = productosCache.find(p => p.id === id);

    if (item.cantidad + delta <= 0) {
        carrito = carrito.filter(i => i.id !== id);
    } else {
        if (item.cantidad + delta > original.stock_actual) {
            alert("⚠️ Stock máximo alcanzado");
            return;
        }
        item.cantidad += delta;
    }
    actualizarVistaCarrito();
}

function vaciarCarrito() {
    if (confirm("¿Vaciar el carrito?")) {
        carrito = [];
        actualizarVistaCarrito();
    }
}

function actualizarVistaCarrito() {
    const list = document.getElementById('pos-cart-list');
    const totalEl = document.getElementById('pos-total');
    const countEl = document.getElementById('pos-items-count');

    if (carrito.length === 0) {
        list.innerHTML = `<p style="text-align:center; padding: 2rem; color: #999;">El carrito está vacío</p>`;
        totalEl.innerText = '$0.00';
        countEl.innerText = 'Productos: 0';
        return;
    }

    let total = 0;
    list.innerHTML = carrito.map(item => {
        const subtotal = item.cantidad * item.precio_unitario;
        total += subtotal;
        return `
            <div class="cart-item">
                <div style="flex-grow: 1;">
                    <strong>${item.nombre}</strong><br>
                    <small>$${item.precio_unitario.toFixed(2)} x ${item.cantidad}</small>
                </div>
                <div style="display: flex; align-items: center; gap: 0.5rem;">
                    <button class="btn-small" onclick="actualizarCantidad('${item.id}', -1)">-</button>
                    <span>${item.cantidad}</span>
                    <button class="btn-small" onclick="actualizarCantidad('${item.id}', 1)">+</button>
                    <strong style="margin-left: 1rem;">$${subtotal.toFixed(2)}</strong>
                </div>
            </div>
        `;
    }).join('');

    totalEl.innerText = `$${total.toFixed(2)}`;
    countEl.innerText = `Productos: ${carrito.length}`;
}

async function finalizarVenta() {
    if (carrito.length === 0) return alert("El carrito está vacío");

    const lineas = carrito.map(item => ({
        producto_id: item.id,
        cantidad: item.cantidad
    }));

    try {
        await apiFetch('/ventas', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ lineas })
        });
        alert("💰 ¡Venta realizada con éxito!");
        cargarPOS(); // Reiniciar POS
    } catch (e) {
        console.error("Venta fallida", e);
    }
}

// --- Módulos Restantes (Similares a los anteriores, actualizados con apiFetch y wrappers) ---

// 📦 PRODUCTOS (Actualizado con API v1)
async function cargarProductos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/productos');
        const productos = result.data;
        main.innerHTML = `
            <div class="header-actions">
                <h2>📦 Productos</h2>
                <button class="btn-primary" onclick="mostrarFormProducto()">+ Nuevo producto</button>
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
                    <tr><th>Código</th><th>Nombre</th><th>Precio</th><th>Stock</th><th>Acciones</th></tr>
                </thead>
                <tbody>
                    ${productos.map(p => `
                        <tr>
                            <td>${p.codigo}</td><td>${p.nombre}</td>
                            <td>$${p.precio_unitario.toFixed(2)}</td>
                            <td><span class="badge ${p.stock_actual <= 5 ? 'badge-danger' : 'badge-success'}">${p.stock_actual}</span></td>
                            <td><button onclick="eliminarProducto('${p.id}')">🗑️</button></td>
                        </tr>
                    `).join('')}
                </tbody>
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
            body: JSON.stringify({
                codigo: e.target.codigo.value,
                nombre: e.target.nombre.value,
                descripcion: e.target.descripcion.value,
                precio_unitario: parseFloat(e.target.precio.value)
            })
        });
        cargarProductos();
    } catch (e) {}
}

async function eliminarProducto(id) {
    if (!confirm('¿Seguro?')) return;
    try {
        await apiFetch(`/productos/${id}`, { method: 'DELETE' });
        cargarProductos();
    } catch (e) {}
}

// 🚛 KARDEX
async function cargarMovimientos() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/inventario/movimientos');
        const movimientos = result.data;
        main.innerHTML = `
            <h2>🚛 Historial de Movimientos (Kardex)</h2>
            <table class="card">
                <thead><tr><th>Fecha</th><th>Tipo</th><th>Cantidad</th><th>Antes</th><th>Después</th><th>Motivo</th></tr></thead>
                <tbody>
                    ${movimientos.map(m => `
                        <tr>
                            <td>${m.fecha.substring(0, 16)}</td>
                            <td><span class="badge ${m.tipo === 'ENTRADA' ? 'badge-success' : m.tipo === 'SALIDA' ? 'badge-danger' : 'badge-warning'}">${m.tipo}</span></td>
                            <td>${m.cantidad}</td><td>${m.stock_antes}</td><td><strong>${m.stock_despues}</strong></td><td>${m.motivo || '-'}</td>
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {}
}

// 💰 HISTORIAL VENTAS
async function cargarVentas() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/ventas');
        const ventas = result.data;
        main.innerHTML = `
            <h2>💰 Historial de Ventas</h2>
            <table class="card">
                <thead><tr><th>Fecha</th><th>Usuario</th><th>Total</th></tr></thead>
                <tbody>
                    ${ventas.map(v => `
                        <tr><td>${v.fecha}</td><td>${v.usuario_id.substring(0,8)}...</td><td>$${v.total.toFixed(2)}</td></tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {}
}

// 👥 USUARIOS
async function cargarUsuarios() {
    const main = document.getElementById('main-content');
    try {
        const result = await apiFetch('/usuarios');
        const usuariosList = result.data;
        main.innerHTML = `
            <h2>👥 Gestión de Usuarios</h2>
            <table class="card">
                <thead><tr><th>Usuario</th><th>Rol</th><th>Cambiar</th></tr></thead>
                <tbody>
                    ${usuariosList.map(u => `
                        <tr><td>${u.username}</td><td><span class="badge">${u.rol}</span></td><td>
                            <select onchange="actualizarRol('${u.id}', this.value)">
                                <option value="usuario" ${u.rol === 'usuario' ? 'selected' : ''}>Usuario</option>
                                <option value="admin" ${u.rol === 'admin' ? 'selected' : ''}>Admin</option>
                            </select>
                        </td></tr>
                    `).join('')}
                </tbody>
            </table>
        `;
    } catch (e) {}
}

async function actualizarRol(id, nuevoRol) {
    try {
        await apiFetch(`/usuarios/${id}/rol`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ rol: nuevoRol })
        });
        alert("Rol actualizado");
    } catch (e) {}
}

// Inicialización
if (token) {
    mostrarDashboard();
} else {
    mostrarLogin();
}
