use reqwest;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("AZURE_OPENAI_API_KEY")?;
    let base_url = env::var("AZURE_BASE_URL")?; // https://your-resource-name.openai.azure.com/openai

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/deployments/gpt-4.1/chat/completions?api-version=2025-01-01-preview", base_url))
        .header("api-key", &api_key)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "messages": [{"role": "user", "content": "test"}],
            "max_tokens": 5
        }))
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Response: {}", response.text().await?);
    Ok(())
}

