#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

// Estructuras de datos con tipado estricto
#[derive(Serialize, Deserialize, Debug)]
struct RegistroIA {
    titulo: String,
    contenido: String,
    embedding: Vec<f32>,
}

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
    options: HFOptions,
}

#[derive(Serialize)]
struct HFOptions {
    wait_for_model: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Configuración de entorno
    let hf_key = env::var("HUGGINGFACE_KEY").expect("Falta HUGGINGFACE_KEY");
    let sb_url = env::var("SUPABASE_URL").expect("Falta SUPABASE_URL");
    let sb_key = env::var("SUPABASE_KEY").expect("Falta SUPABASE_KEY");

    let client = reqwest::Client::new();
    let ruta_docs = "./documentos";

    // Verificación de carpeta local
    if !Path::new(ruta_docs).exists() {
        fs::create_dir(ruta_docs)?;
        println!("📂 Carpeta './documentos' creada. Coloca tus .txt allí.");
        return Ok(());
    }

    let archivos = fs::read_dir(ruta_docs)?;

    for archivo in archivos {
        let entrada = archivo?;
        let nombre = entrada.file_name().into_string().unwrap();

        // Solo procesamos archivos .txt para ahorrar recursos
        if !nombre.ends_with(".txt") {
            continue;
        }

        let texto = leer_y_limpiar_archivo(entrada.path())?;
        println!("🧠 Procesando: {}", nombre);

        // 1. Obtener Embedding (IA Externa para no estresar tu CPU)
        let embedding = obtener_embedding(&client, &hf_key, &texto).await?;

        // 2. Cargar en el Cerebro (Supabase)
        enviar_a_supabase(
            &client,
            &sb_url,
            &sb_key,
            RegistroIA {
                titulo: nombre.clone(),
                contenido: texto,
                embedding,
            },
        )
        .await?;

        println!("✅ {} indexado con éxito.", nombre);
    }

    Ok(())
}

// --- FUNCIONES MODULARES ---

fn leer_y_limpiar_archivo<P: AsRef<Path>>(ruta: P) -> Result<String, std::io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    // Limpieza básica: quitar espacios extra para Green Computing (menos bytes)
    Ok(contenido.trim().replace("\r\n", "\n"))
}

async fn obtener_embedding(
    client: &reqwest::Client,
    key: &str,
    texto: &str,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let model = "sentence-transformers/all-MiniLM-L6-v2";
    let url = format!(
        "https://api-inference.huggingface.co/pipeline/feature-extraction/{}",
        model
    );

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .json(&HFRequest {
            inputs: texto.to_string(),
            options: HFOptions {
                wait_for_model: true,
            },
        })
        .send()
        .await?
        .json::<Vec<f32>>()
        .await?;

    Ok(res)
}

async fn enviar_a_supabase(
    client: &reqwest::Client,
    url: &str,
    key: &str,
    registro: RegistroIA,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .post(format!("{}/rest/v1/conocimiento_agente", url))
        .header("apikey", key)
        .header("Authorization", format!("Bearer {}", key))
        .header("Content-Type", "application/json")
        .json(&registro)
        .send()
        .await?;
    Ok(())
}

// --- SECCIÓN DE PRUEBAS UNITARIAS (TDD) ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_leer_y_limpiar_archivo() {
        // 1. Crear un archivo temporal de prueba
        let test_path = "test_doc.txt";
        let mut file = fs::File::create(test_path).unwrap();
        writeln!(file, "  Hola Mundo \r\n").unwrap();

        // 2. Ejecutar la función
        let resultado = leer_y_limpiar_archivo(test_path).unwrap();

        // 3. Verificar (Assert)
        // Esperamos que quite los espacios de los extremos y el salto de línea de Windows
        assert_eq!(resultado, "Hola Mundo");

        // Limpiar después del test
        fs::remove_file(test_path).unwrap();
    }
}
