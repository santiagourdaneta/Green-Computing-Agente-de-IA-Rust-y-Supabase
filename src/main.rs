#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

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

    let hf_key = env::var("HUGGINGFACE_KEY").expect("Falta HUGGINGFACE_KEY");
    let sb_url = env::var("SUPABASE_URL").expect("Falta SUPABASE_URL");
    let sb_key = env::var("SUPABASE_KEY").expect("Falta SUPABASE_KEY");

    let client = reqwest::Client::new();
    let ruta_docs = "./documentos";

    let archivos = fs::read_dir(ruta_docs)?;

    for archivo in archivos {
        let entrada = archivo?;
        let nombre_os = entrada.file_name();
        let nombre = nombre_os.to_string_lossy();

        if !Path::new(&*nombre)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("txt"))
        {
            continue;
        }

        let texto = leer_y_limpiar_archivo(entrada.path())?;
        println!("🧠 Procesando: {nombre}");

        let embedding = obtener_embedding(&client, &hf_key, &texto).await?;

        enviar_a_supabase(
            &client,
            &sb_url,
            &sb_key,
            RegistroIA {
                titulo: nombre.to_string(),
                contenido: texto,
                embedding,
            },
        )
        .await?;

        println!("✅ {nombre} indexado con éxito.");
    }

    Ok(())
}

fn leer_y_limpiar_archivo<P: AsRef<Path>>(ruta: P) -> Result<String, std::io::Error> {
    let contenido = fs::read_to_string(ruta)?;
    Ok(contenido.trim().replace("\r\n", "\n"))
}

async fn obtener_embedding(
    client: &reqwest::Client,
    key: &str,
    texto: &str,
) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let model = "sentence-transformers/all-MiniLM-L6-v2";
    let url = format!("https://api-inference.huggingface.co/pipeline/feature-extraction/{model}");

    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {key}"))
        .json(&HFRequest {
            inputs: texto.to_string(),
            options: HFOptions {
                wait_for_model: true,
            },
        })
        .send()
        .await?;

    if !res.status().is_success() {
        let error_text = res.text().await?;
        return Err(format!("HF API Error: {error_text}").into());
    }

    let embedding = res.json::<Vec<f32>>().await?;
    Ok(embedding)
}

async fn enviar_a_supabase(
    client: &reqwest::Client,
    url: &str,
    key: &str,
    registro: RegistroIA,
) -> Result<(), Box<dyn std::error::Error>> {
    client
        .post(format!("{url}/rest/v1/conocimiento_agente"))
        .header("apikey", key)
        .header("Authorization", format!("Bearer {key}"))
        .header("Content-Type", "application/json")
        .json(&registro)
        .send()
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_leer_y_limpiar_archivo() {
        let test_path = "test_doc.txt";
        let mut file = fs::File::create(test_path).unwrap();
        writeln!(file, "  Hola Mundo \r\n").unwrap();
        let resultado = leer_y_limpiar_archivo(test_path).unwrap();
        assert_eq!(resultado, "Hola Mundo");
        fs::remove_file(test_path).unwrap();
    }
}
