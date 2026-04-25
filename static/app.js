let token = localStorage.getItem('token');
let usuarioActual = JSON.parse(localStorage.getItem('usuario')) || null;

async function api(url, method = 'GET', body = null) {
    const headers = { 'Content-Type': 'application/json' };
    if (token) headers['Authorization'] = `Bearer ${token}`;
    const res = await fetch(url, { method, headers, body: body ? JSON.stringify(body) : null });
    if (res.status === 401) { cerrarSesion(); throw new Error('Sesión expirada'); }
    if (!res.ok) {
        const err = await res.text();
        alert(`Error: ${err}`);
        throw new Error(err);
    }
    return res.json();
}

// ---------- Autenticación ----------
async function login() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;
    if (!username || !password) return alert('Completa los campos');
    const data = await api('/api/login', 'POST', { username, password });
    token = data.token;
    usuarioActual = data.usuario;
    localStorage.setItem('token', token);
    localStorage.setItem('usuario', JSON.stringify(usuarioActual));
    mostrarApp();
}

async function registro() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;
    if (!username || !password) return alert('Completa los campos');
    const data = await api('/api/registro', 'POST', { username, password });
    token = data.token;
    usuarioActual = data.usuario;
    localStorage.setItem('token', token);
    localStorage.setItem('usuario', JSON.stringify(usuarioActual));
    mostrarApp();
}

function cerrarSesion() {
    token = null;
    usuarioActual = null;
    localStorage.removeItem('token');
    localStorage.removeItem('usuario');
    document.getElementById('login-section').style.display = 'block';
    document.getElementById('main-section').style.display = 'none';
}

function mostrarApp() {
    document.getElementById('login-section').style.display = 'none';
    document.getElementById('main-section').style.display = 'block';
    
    // Mostrar/ocultar menú de usuarios según rol
    const btnUsuarios = document.getElementById('btn-usuarios');
    if (usuarioActual && usuarioActual.rol === 'admin') {
        btnUsuarios.style.display = 'inline-block';
    } else {
        btnUsuarios.style.display = 'none';
    }
    
    mostrarSeccion('productos');
}

// ---------- Navegación ----------
function mostrarSeccion(seccion) {
    if (seccion === 'productos') cargarProductos();
    else if (seccion === 'ventas') cargarVentas();
    else if (seccion === 'movimientos') cargarMovimientos();
    else if (seccion === 'usuarios') cargarUsuarios();
}

// ---------- Productos ----------
async function cargarProductos() {
    const productos = await api('/api/productos');
    let html = `
        <div class="header-actions">
            <h2>📦 Productos</h2>
            <div class="botones-header">
                <label style="font-size: 0.8rem; color: var(--secondary); margin-right: 1rem;">
                    <input type="checkbox" id="check-stock-bajo" style="width: auto;"> Solo stock bajo (≤5)
                </label>
                <button class="btn-primary" onclick="mostrarFormProducto()" style="width: auto;">+ Nuevo producto</button>
                <button class="btn-secondary" onclick="descargarReporteInventario()" style="width: auto;">📄 Exportar PDF</button>
            </div>
        </div>
        <div id="form-producto" class="card" style="display:none; margin-bottom: 2rem;"></div>
        <table>
            <thead>
                <tr><th>Código</th><th>Nombre</th><th>Precio</th><th>Stock</th><th>Acciones</th></tr>
            </thead>
            <tbody>
    `;
    productos.forEach(p => {
        html += `<tr>
            <td>${p.codigo}</td>
            <td>${p.nombre}</td>
            <td>$${p.precio_unitario.toFixed(2)}</td>
            <td>${p.stock_actual}</td>
            <td class="actions-cell">
                <button class="btn-secondary" onclick="editarProducto('${p.id}')">Editar</button>
                <button class="btn-danger" onclick="eliminarProducto('${p.id}')">Eliminar</button>
            </td>
        </tr>`;
    });
    html += '</tbody></table>';
    document.getElementById('contenido').innerHTML = html;
}

function mostrarFormProducto(producto = null) {
    const formDiv = document.getElementById('form-producto');
    const esEdicion = producto !== null;
    formDiv.style.display = 'block';
    formDiv.innerHTML = `
        <h3>${esEdicion ? '✏️ Editar producto' : '✨ Nuevo producto'}</h3>
        <div class="form-group"><input type="text" id="prod-codigo" placeholder="Código" value="${esEdicion ? producto.codigo : ''}"></div>
        <div class="form-group"><input type="text" id="prod-nombre" placeholder="Nombre" value="${esEdicion ? producto.nombre : ''}"></div>
        <div class="form-group"><input type="text" id="prod-descripcion" placeholder="Descripción" value="${esEdicion ? (producto.descripcion || '') : ''}"></div>
        <div class="form-group"><input type="number" id="prod-precio" placeholder="Precio" step="0.01" value="${esEdicion ? producto.precio_unitario : ''}"></div>
        <div class="actions-cell">
            <button class="btn-primary" onclick="${esEdicion ? `guardarEdicionProducto('${producto.id}')` : 'crearProducto()'}">Guardar</button>
            <button class="btn-secondary" onclick="document.getElementById('form-producto').style.display='none'">Cancelar</button>
        </div>
    `;
}

async function crearProducto() {
    const codigo = document.getElementById('prod-codigo').value.trim();
    const nombre = document.getElementById('prod-nombre').value.trim();
    const descripcion = document.getElementById('prod-descripcion').value.trim();
    const precio = parseFloat(document.getElementById('prod-precio').value);
    if (!codigo || !nombre || isNaN(precio)) return alert('Campos obligatorios');
    await api('/api/productos', 'POST', { codigo, nombre, descripcion: descripcion || null, precio_unitario: precio });
    cargarProductos();
}

async function editarProducto(id) {
    const productos = await api('/api/productos');
    const prod = productos.find(p => p.id === id);
    if (prod) mostrarFormProducto(prod);
}

async function guardarEdicionProducto(id) {
    const codigo = document.getElementById('prod-codigo').value.trim();
    const nombre = document.getElementById('prod-nombre').value.trim();
    const descripcion = document.getElementById('prod-descripcion').value.trim();
    const precio = parseFloat(document.getElementById('prod-precio').value);
    const body = {};
    if (codigo) body.codigo = codigo;
    if (nombre) body.nombre = nombre;
    if (descripcion) body.descripcion = descripcion;
    if (!isNaN(precio)) body.precio_unitario = precio;
    await api(`/api/productos/${id}`, 'PUT', body);
    cargarProductos();
}

async function eliminarProducto(id) {
    if (!confirm('¿Eliminar producto?')) return;
    await api(`/api/productos/${id}`, 'DELETE');
    cargarProductos();
}

// ---------- Ventas ----------
async function cargarVentas() {
    let html = `
        <div class="header-actions">
            <h2>💰 Ventas</h2>
            <div class="botones-header" style="display: flex; gap: 0.5rem; align-items: center;">
                <input type="month" id="reporte-mes" value="${new Date().toISOString().slice(0, 7)}" style="width: auto; margin: 0;">
                <button class="btn-secondary" onclick="descargarReporteVentas()" style="width: auto; margin: 0;">📊 Reporte Mensual PDF</button>
                <button class="btn-primary" onclick="mostrarFormVenta()" style="width: auto; margin: 0;">+ Nueva venta</button>
            </div>
        </div>
        <div id="form-venta" class="card" style="display:none; margin-bottom: 2rem;">
            <h3>Nueva venta</h3>
            <div id="lineas-venta"></div>
            <button class="btn-secondary" onclick="agregarLineaVenta()" style="width: auto; margin-bottom: 1rem;">+ Agregar producto</button>
            <div style="margin-bottom: 1.5rem; font-size: 1.25rem;">
                <strong>Total: $<span id="total-venta">0.00</span></strong>
            </div>
            <div class="actions-cell">
                <button class="btn-primary" onclick="realizarVenta()">Finalizar venta</button>
                <button class="btn-secondary" onclick="document.getElementById('form-venta').style.display='none'">Cancelar</button>
            </div>
        </div>
        <table>
            <thead><tr><th>ID</th><th>Total</th><th>Fecha</th><th>Detalle</th></tr></thead>
            <tbody id="tabla-ventas-body"></tbody>
        </table>
    `;
    document.getElementById('contenido').innerHTML = html;

    const ventas = await api('/api/ventas');
    const tbody = document.getElementById('tabla-ventas-body');
    ventas.forEach(v => {
        const fila = tbody.insertRow();
        fila.innerHTML = `<td>${v.id.substring(0,8)}...</td><td>$${v.total.toFixed(2)}</td><td>${v.fecha}</td><td><button class="btn-secondary" onclick="alert('Detalle ID: ${v.id}')">Ver</button></td>`;
    });
}

let lineasVenta = []; 

async function mostrarFormVenta() {
    document.getElementById('form-venta').style.display = 'block';
    lineasVenta = [];
    agregarLineaVenta();
}

async function agregarLineaVenta() {
    lineasVenta.push({ producto_id: '', cantidad: 1 });
    actualizarLineasVenta();
}

async function actualizarLineasVenta() {
    const productos = await api('/api/productos');
    const lineasDiv = document.getElementById('lineas-venta');
    lineasDiv.innerHTML = lineasVenta.map((l, i) => `
        <div class="form-group" style="display: flex; gap: 1rem; align-items: center; margin-bottom: 1rem;">
            <select style="flex: 2;" onchange="lineasVenta[${i}].producto_id = this.value; calcularTotalVenta()">
                <option value="">Seleccione producto</option>
                ${productos.map(p => `<option value="${p.id}" ${p.id === l.producto_id ? 'selected' : ''}>${p.nombre} ($${p.precio_unitario} - Stock: ${p.stock_actual})</option>`).join('')}
            </select>
            <input style="flex: 1;" type="number" min="1" value="${l.cantidad}" onchange="lineasVenta[${i}].cantidad = parseInt(this.value); calcularTotalVenta()">
            <button class="btn-danger" style="width: auto;" onclick="lineasVenta.splice(${i},1); actualizarLineasVenta()">🗑️</button>
        </div>
    `).join('');
    calcularTotalVenta();
}

async function calcularTotalVenta() {
    const productos = await api('/api/productos');
    const prodMap = {};
    productos.forEach(p => prodMap[p.id] = p);
    let total = 0;
    lineasVenta.forEach(l => {
        if (l.producto_id && prodMap[l.producto_id]) {
            total += prodMap[l.producto_id].precio_unitario * l.cantidad;
        }
    });
    document.getElementById('total-venta').textContent = total.toFixed(2);
}

async function realizarVenta() {
    const lineas = lineasVenta.filter(l => l.producto_id && l.cantidad > 0);
    if (lineas.length === 0) return alert('Agrega productos válidos');
    try {
        await api('/api/ventas', 'POST', { lineas });
        alert('Venta registrada con éxito');
        document.getElementById('form-venta').style.display = 'none';
        cargarVentas();
    } catch (e) {
        console.error(e);
    }
}

// ---------- Movimientos ----------
async function cargarMovimientos() {
    const productos = await api('/api/productos');
    let html = `
        <h2>📊 Historial de movimientos</h2>
        <div class="card" style="margin-bottom: 2rem;">
            <div class="form-group" style="display: flex; gap: 1rem; flex-wrap: wrap;">
                <select id="filtro-producto" style="flex: 1; min-width: 200px;">
                    <option value="">Todos los productos</option>
                    ${productos.map(p => `<option value="${p.id}">${p.nombre}</option>`).join('')}
                </select>
                <select id="filtro-tipo" style="flex: 1; min-width: 150px;">
                    <option value="">Todos los tipos</option>
                    <option value="entrada">🟢 Entrada</option>
                    <option value="salida">🔴 Salida</option>
                </select>
                <input type="date" id="filtro-desde" style="flex: 1; min-width: 150px;">
                <input type="date" id="filtro-hasta" style="flex: 1; min-width: 150px;">
                <button class="btn-primary" onclick="aplicarFiltrosMovimientos()" style="width: auto;">Filtrar</button>
            </div>
        </div>
        <div id="movimientos-tabla-container"></div>
    `;
    document.getElementById('contenido').innerHTML = html;
    aplicarFiltrosMovimientos();
}

async function aplicarFiltrosMovimientos() {
    const producto_id = document.getElementById('filtro-producto')?.value || '';
    const tipo = document.getElementById('filtro-tipo')?.value || '';
    const desde = document.getElementById('filtro-desde')?.value || '';
    const hasta = document.getElementById('filtro-hasta')?.value || '';

    const params = new URLSearchParams();
    if (producto_id) params.append('producto_id', producto_id);
    if (tipo) params.append('tipo', tipo);
    if (desde) params.append('fecha_desde', desde);
    if (hasta) params.append('fecha_hasta', hasta);

    const movimientos = await api('/api/inventario/movimientos?' + params.toString());
    const productos = await api('/api/productos');
    const prodMap = {};
    productos.forEach(p => prodMap[p.id] = p.nombre);

    let tablaHtml = `
        <table>
            <thead><tr><th>Producto</th><th>Tipo</th><th>Cantidad</th><th>Motivo</th><th>Fecha</th></tr></thead>
            <tbody>
    `;
    movimientos.forEach(m => {
        tablaHtml += `<tr>
            <td>${prodMap[m.producto_id] || m.producto_id}</td>
            <td>${m.tipo === 'entrada' ? '<span style="color:var(--success)">🟢 Entrada</span>' : '<span style="color:var(--danger)">🔴 Salida</span>'}</td>
            <td>${m.cantidad}</td>
            <td>${m.motivo || '-'}</td>
            <td>${m.fecha}</td>
        </tr>`;
    });
    tablaHtml += '</tbody></table>';
    document.getElementById('movimientos-tabla-container').innerHTML = tablaHtml;
}

// ---------- Usuarios (Admin) ----------
async function cargarUsuarios() {
    const usuarios = await api('/api/usuarios');
    let html = `
        <h2>👥 Gestión de Usuarios</h2>
        <table>
            <thead><tr><th>ID</th><th>Usuario</th><th>Rol</th><th>Acciones</th></tr></thead>
            <tbody>
    `;
    usuarios.forEach(u => {
        html += `<tr>
            <td>${u.id.substring(0,8)}...</td>
            <td>${u.username}</td>
            <td><strong>${u.rol.toUpperCase()}</strong></td>
            <td class="actions-cell">
                <select onchange="cambiarRol('${u.id}', this.value)">
                    <option value="usuario" ${u.rol === 'usuario' ? 'selected' : ''}>Usuario</option>
                    <option value="admin" ${u.rol === 'admin' ? 'selected' : ''}>Admin</option>
                </select>
            </td>
        </tr>`;
    });
    html += '</tbody></table>';
    document.getElementById('contenido').innerHTML = html;
}

async function cambiarRol(userId, nuevoRol) {
    if (!confirm(`¿Cambiar rol a ${nuevoRol}?`)) return cargarUsuarios();
    await api(`/api/usuarios/${userId}/rol`, 'PUT', { rol: nuevoRol });
    alert('Rol actualizado');
    if (userId === usuarioActual.id) {
        usuarioActual.rol = nuevoRol;
        localStorage.setItem('usuario', JSON.stringify(usuarioActual));
        mostrarApp();
    } else {
        cargarUsuarios();
    }
}

// ---------- Reportes ----------
async function descargarReporteInventario() {
    const soloStockBajo = document.getElementById('check-stock-bajo')?.checked;
    const urlParams = soloStockBajo ? '?solo_stock_bajo=true&stock_minimo=5' : '';
    
    try {
        const response = await fetch('/api/reportes/inventario' + urlParams, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (!response.ok) throw new Error(await response.text());
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `reporte_inventario${soloStockBajo ? '_stock_bajo' : ''}.pdf`;
        document.body.appendChild(a);
        a.click();
        a.remove();
    } catch (e) {
        alert('Error al generar PDF.');
        console.error(e);
    }
}

async function descargarReporteVentas() {
    const inputMes = document.getElementById('reporte-mes').value;
    if (!inputMes) return alert('Selecciona un mes');
    const [anio, mes] = inputMes.split('-');
    
    try {
        const response = await fetch(`/api/reportes/ventas?anio=${anio}&mes=${mes}`, {
            headers: { 'Authorization': `Bearer ${token}` }
        });
        if (!response.ok) throw new Error(await response.text());
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `reporte_ventas_${anio}_${mes}.pdf`;
        document.body.appendChild(a);
        a.click();
        a.remove();
    } catch (e) {
        alert('Error al generar reporte de ventas.');
        console.error(e);
    }
}

// Iniciar app
if (token) mostrarApp();
else cerrarSesion();
