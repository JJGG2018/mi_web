use axum::extract::Form;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use tower_http::services::ServeDir;

#[derive(Deserialize, Serialize, Clone)]
struct Contacto {
    nombre: String,
    email: String,
    mensaje: String,
}

async fn portfolio() -> Html<String> {
    let html = std::fs::read_to_string("templates/index.html")
        .unwrap_or_else(|_| "Error cargando portfolio".to_string());
    Html(html)
}

async fn recibir_formulario(Form(contacto): Form<Contacto>) -> Html<String> {
    let directorio = "data";
    let ruta = "data/contactos.json";

    // Intentar guardar el contacto (sin colapsar si falla el sistema de archivos)
    let resultado_guardado = async {
        // Asegurarse de que la carpeta 'data' existe
        if !Path::new(directorio).exists() {
            fs::create_dir_all(directorio).await?;
        }
        // Si el archivo no existe, créalo con []
        if !Path::new(ruta).exists() {
            fs::write(ruta, "[]").await?;
        }

        let contenido = fs::read_to_string(ruta)
            .await
            .unwrap_or_else(|_| "[]".to_string());
        let mut lista: Vec<Contacto> = serde_json::from_str(&contenido).unwrap_or_default();
        lista.push(contacto.clone());

        let nuevo_json = serde_json::to_string_pretty(&lista)?;
        fs::write(ruta, nuevo_json).await?;
        Ok::<(), Box<dyn std::error::Error>>(())
    }
    .await;

    if let Err(e) = resultado_guardado {
        eprintln!("Error guardando contacto: {}", e);
    }

    // Respuesta visualmente atractiva (Bootstrap + Estilos)
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="es">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Mensaje Recibido</title>
            <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet">
            <style>
                body {{ background: #f8f9fa; height: 100vh; display: flex; align-items: center; justify-content: center; }}
                .card {{ border: none; border-radius: 15px; box-shadow: 0 10px 30px rgba(0,0,0,0.1); padding: 2rem; max-width: 500px; text-align: center; }}
                .icon {{ font-size: 4rem; color: #198754; margin-bottom: 1rem; }}
            </style>
        </head>
        <body>
            <div class="card">
                <div class="icon">✓</div>
                <h1 class="display-6">¡Gracias, {}!</h1>
                <p class="lead text-muted">Tu mensaje ha sido recibido correctamente. Te contactaré pronto.</p>
                <hr>
                <a href="/" class="btn btn-primary px-4 py-2">Volver al inicio</a>
            </div>
        </body>
        </html>
        "#,
        contacto.nombre
    );

    Html(html)
}

// Versión para Shuttle
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(portfolio))
        .route("/enviar", post(recibir_formulario))
        .nest_service("/static", ServeDir::new("static"));

    Ok(router.into())
}

// Versión para Railway/Docker/otros
#[cfg(not(feature = "shuttle"))]
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(portfolio))
        .route("/enviar", post(recibir_formulario))
        .nest_service("/static", ServeDir::new("static"));

    // Leer puerto de variable de entorno (Railway usa PORT)
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Servidor corriendo en http://0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
