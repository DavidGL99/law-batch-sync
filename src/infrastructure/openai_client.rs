// infrastructure/open_api.rs
use std::env;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
}

pub struct OpenAIClient {
    api_key: String,
    client: reqwest::Client,
}

impl OpenAIClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY no está definido");
        let client = reqwest::Client::new();
        OpenAIClient { api_key, client }
    }

    pub async fn chat(&self, texto_legal: &str) -> Result<String, Box<dyn std::error::Error>> {
        let formato = "[ { \"tipo\": \"prision | multa\", \"min\": \"...\", \"max\": \"...\", \"condicion\": \"...\" } ]";

        let prompt = format!(
            "{} Extrae todas las penas del texto y devuélvelas en JSON con este formato: {}",
            texto_legal, formato
        );


        let request_body = ChatRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let res = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        // Primero parseamos como JSON genérico para debug
        let json: Value = res.json().await?;
        // println!("{:#}", json); // <- Descomenta para ver la respuesta completa

        // Intentamos extraer la primera respuesta si existe
        if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else if let Some(error_msg) = json["error"]["message"].as_str() {
            Err(format!("OpenAI API error: {}", error_msg).into())
        } else {
            Err("Respuesta inesperada de la API de OpenAI".into())
        }
    }
}
