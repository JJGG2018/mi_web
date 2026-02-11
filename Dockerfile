# Etapa de compilación
FROM rust:1.85 as builder

WORKDIR /app

# Copiar manifiestos
COPY Cargo.toml Cargo.lock ./

# Crear un proyecto dummy para cachear dependencias
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copiar el código real
COPY src ./src
COPY templates ./templates
COPY static ./static

# Compilar la aplicación
RUN touch src/main.rs && cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

WORKDIR /app

# Instalar dependencias de runtime
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado
COPY --from=builder /app/target/release/mi_web /app/mi_web

# Copiar archivos estáticos y templates
COPY templates ./templates
COPY static ./static

# Crear directorio para datos
RUN mkdir -p /app/data

# Exponer el puerto
EXPOSE 8000

# Ejecutar la aplicación
CMD ["/app/mi_web"]
