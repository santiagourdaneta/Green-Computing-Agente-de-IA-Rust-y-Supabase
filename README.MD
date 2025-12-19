# 🦾 Green Computing Agente de IA

> Una arquitectura RAG (Retrieval-Augmented Generation) de alto impacto, construida para hardware limitado bajo principios de **Green Software Engineering**.

[![Quality_Gate_Senior](https://img.shields.io/badge/Quality_Gate-Passed-00ff9d?style=for-the-badge&logo=github-actions)](https://github.com)
[![Rust_Inside](https://img.shields.io/badge/Engine-Rust-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Vercel_Edge](https://img.shields.io/badge/Cloud-Vercel_Edge-black?style=for-the-badge&logo=vercel)](https://vercel.com)

---

## 🎧 El Concepto: "Architectural Turntablism"

Al igual que un productor de Hip Hop toma un *sample* clásico para crear un hit moderno, este proyecto toma **hardware antiguo** y lo potencia mediante:

1.  **Rust Engine:** Procesamiento de datos de bajo consumo y latencia cero.
2.  **Edge Intelligence:** Inferencia de IA delegada a la nube (HuggingFace/Groq) para proteger la RAM local.
3.  **Vector Memory:** Búsqueda semántica en Supabase para entender ideas, no solo palabras.

---

## 🛠️ Stack Tecnológico

### **Backend & Core**
* **Rust:** Indexador de alta eficiencia y bajo consumo de memoria.
* **HuggingFace API:** Generación distribuida de Embeddings (Modelo: `all-MiniLM-L6-v2`).
* **Supabase:** Base de datos vectorial con `pgvector`.

### **Frontend & UI/UX**
* **HTML5/CSS3 (Vanilla):** Zero-framework para carga instantánea en navegadores antiguos.
* **SEO:** Meta tags OpenGraph y Twitter Cards integrados.
* **Accesibilidad:** Cumplimiento de estándares ARIA y contraste optimizado.

### **DevOps & Calidad**
* **GitHub Actions:** Pipeline de CI/CD con **Quality Gates**.
* **Docker:** Multi-stage builds para contenedores ultra-ligeros (<100MB).
* **K6:** Pruebas de estrés automatizadas.

---

## 🛡️ Quality Gate (The Guardian)
Este repositorio está blindado. El robot de CI revisa:

* **Linter (`cargo clippy`):** Prohíbe código ineficiente o inseguro.
* **Formatter (`cargo fmt`):** Garantiza un estilo de código estándar global.
* **Unit & Integration Tests:** Validación de lógica y conexiones con la API.
* **Stress Testing:** Simulación de carga para prevenir caídas bajo presión.

---

## 🚀 Instalación y Despliegue

### 1. Variables de Entorno
Crea un archivo `.env` en la raíz con tus credenciales:
```text
SUPABASE_URL=[https://tu-proyecto.supabase.co](https://tu-proyecto.supabase.co)
SUPABASE_KEY=tu-api-key
HUGGINGFACE_KEY=tu-token-hf

2. Ejecutar el Motor (Rust)

# Formatear, revisar y correr
cargo fmt
cargo clippy
cargo run

3. Ejecutar en Contenedor (Docker)

docker compose up --build

🌍 Compromiso Green Computing

Limpieza de Datos: Rust procesa y limpia los archivos localmente antes de subirlos, reduciendo el tráfico de red en un 20%.

Eficiencia Energética: El uso de lenguajes compilados reduce el tiempo de CPU, minimizando la huella de carbono del software.

Arquitectura Edge: Evitar que el hardware local se sobrecaliente delegando tareas pesadas de forma inteligente.

Desarrollado con una mentalidad de arquitectura robusta y moderna. 



¿Quieres llevar este núcleo al siguiente nivel? ¡Hablemos!



