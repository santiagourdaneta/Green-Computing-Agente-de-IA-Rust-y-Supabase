#[tokio::test]
async fn test_conexion_supabase_real() {
    dotenv::dotenv().ok();
    let client = reqwest::Client::new();
    let url = std::env::var("SUPABASE_URL").unwrap();

    // Intentamos leer la tabla (sin importar si hay datos)
    let res = client
        .get(format!("{}/rest/v1/conocimiento_agente?select=*", url))
        .header("apikey", std::env::var("SUPABASE_KEY").unwrap())
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}
