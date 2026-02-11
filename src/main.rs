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

    // Asegurarse de que la carpeta 'data' existe
    if !Path::new(directorio).exists() {
        fs::create_dir_all(directorio).await.unwrap();
    }
    // Si el archivo no existe, créalo con []
    if !Path::new(ruta).exists() {
        fs::write(ruta, "[]").await.unwrap();
    }

    // 1. Leer contenido actual (si falla la lectura, usamos una lista vacía en texto)
    let contenido = fs::read_to_string(ruta)
        .await
        .unwrap_or_else(|_| "[]".to_string());

    // 2. Intentar parsear el JSON. Si el archivo está vacío o mal formado, usamos una lista vacía.
    let mut lista: Vec<Contacto> = serde_json::from_str(&contenido).unwrap_or_default();
    // Añadir nuevo contacto
    lista.push(contacto.clone());

    // Guardar actualizado
    let nuevo_json = serde_json::to_string_pretty(&lista).unwrap();
    fs::write(ruta, nuevo_json).await.unwrap();

    // Respuesta al usuario
    let html = format!(
        "<h1>¡Gracias, {}!</h1><p>Tu mensaje ha sido guardado correctamente.</p>",
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
