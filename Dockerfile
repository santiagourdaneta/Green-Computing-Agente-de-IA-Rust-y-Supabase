# ETAPA 1: Construcción (La cocina pesada)
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
# Construimos la versión final optimizada
RUN cargo build --release

# ETAPA 2: Ejecución (El plato servido - Ultra ligero)
FROM debian:bookworm-slim
WORKDIR /app
# Solo traemos el ejecutable, nada de basura extra
COPY --from=builder /app/target/release/indexador-ia .
COPY --from=builder /app/.env .
# Creamos la carpeta de documentos por si no existe
RUN mkdir documentos

CMD ["./indexador-ia"]