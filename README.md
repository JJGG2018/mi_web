# Mi Web - Portfolio Personal

Aplicación web en Rust usando Axum para mostrar un portfolio y recibir mensajes de contacto.

## 🚀 Despliegue

Esta aplicación está preparada para desplegarse en múltiples plataformas:

### Opción 1: Shuttle.dev (Recomendado para Rust)

```bash
# Instalar Shuttle CLI
cargo install cargo-shuttle

# Login
shuttle login

# Iniciar proyecto
shuttle project start

# Desplegar
cargo shuttle deploy --features shuttle
```

### Opción 2: Railway.app

1. Ve a [railway.app](https://railway.app)
2. Crea una cuenta y conecta tu repositorio de GitHub
3. Railway detectará automáticamente el Dockerfile
4. El despliegue será automático

**Variables de entorno necesarias:**
- `PORT` - Railway lo configura automáticamente

### Opción 3: Fly.io

```bash
# Instalar flyctl
# En Windows: iwr https://fly.io/install.ps1 -useb | iex

# Login
fly auth login

# Lanzar aplicación
fly launch

# Desplegar
fly deploy
```

### Opción 4: Render.com

1. Ve a [render.com](https://render.com)
2. Conecta tu repositorio
3. Selecciona "Docker" como tipo de servicio
4. Render usará el Dockerfile automáticamente

## 🏃 Ejecución Local

### Con Shuttle:
```bash
shuttle run --features shuttle
```

### Sin Shuttle (modo tradicional):
```bash
cargo run
# La aplicación estará en http://localhost:8000
```

## 📁 Estructura del Proyecto

```
mi_web/
├── src/
│   └── main.rs          # Código principal
├── templates/
│   └── index.html       # Template del portfolio
├── static/              # Archivos estáticos (CSS, JS, imágenes)
├── data/
│   └── contactos.json   # Almacenamiento de contactos
├── Cargo.toml           # Dependencias
├── Dockerfile           # Para despliegue en Railway/Fly/Render
└── README.md
```

## ⚠️ Nota sobre Persistencia

Los contactos se guardan en `data/contactos.json`. En despliegues con contenedores (Railway, Fly, Render), este archivo **no persiste** entre reinicios.

Para persistencia real, considera migrar a:
- Shuttle Persist (clave-valor simple)
- Shuttle Postgres / Railway Postgres
- Cualquier base de datos externa

## 🔧 Tecnologías

- **Rust** - Lenguaje de programación
- **Axum** - Framework web
- **Tokio** - Runtime asíncrono
- **Serde** - Serialización JSON
- **Tower-HTTP** - Servir archivos estáticos
