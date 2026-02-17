// infrastructure/open_api.rs
use std::env;
use serde::{Serialize};
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
        let formato = r#"
        {
        "version_modelo": "motor_penal_v1.0",
        "consecuencias_juridicas": [
            {
            "id": "string",
            "clase": "pena",
            "naturaleza": "prision | multa | inhabilitacion | otra",
            "estructura": {
                "tipo_duracion": "rango | fija",
                "duracion": {
                "min": "number | null",
                "max": "number | null",
                "unidad": "meses | dias | años | null"
                }
            },
            "aplicacion": {
                "modo": "obligatoria | alternativa | subsidiaria",
                "condicion": {
                "descripcion": "string"
                }
            }
            }
        ],
        "reglas_de_determinacion": [
            {
            "id": "string",
            "categoria": "agravacion | atenuacion | regla_especial",
            "operacion": {
                "tipo": "fraccion_rango | grado | sustitucion",
                "subtipo": "mitad_superior | mitad_inferior | grado_superior | grado_inferior | null"
            },
            "aplica_a": "id_de_consecuencia",
            "condicion": {
                "descripcion": "string"
            }
            }
        ]
        }
        "#;

        let prompt = format!(
        r#"
        Eres un sistema experto en técnica penal y determinación de penas.

        TAREA:
        Extrae exclusivamente consecuencias_juridicas y reglas_de_determinacion del texto.

        REGLAS ESTRICTAS (OBLIGATORIAS):

        1. Si el texto establece "pena superior en grado" o "pena inferior en grado":
        - NO crees una nueva consecuencia_juridica.
        - NO dupliques la pena base.
        - Debe generarse UNA regla_de_determinacion.
        - operacion.tipo = "grado"
        - operacion.subtipo = "grado_superior" o "grado_inferior"
        - aplica_a debe referenciar el id de la pena base.

        2. Si el texto establece "mitad superior" o "mitad inferior":
        - NO crees una nueva pena.
        - Debe generarse UNA regla_de_determinacion.
        - operacion.tipo = "fraccion_rango"

        3. Solo debe existir una consecuencia_juridica por cada pena base autónoma.

        4. Una modificación del grado o fracción NUNCA es una pena independiente.

        5. Devuelve SOLO JSON válido.
        6. No incluyas explicaciones.
        7. No inventes datos.
        8. Si no puede estructurarse correctamente devuelve:
        {{ "error": "no_extraible" }}

        MODELO_JSON_OBJETIVO:
        {formato}

        TEXTO_LEGAL:
        {texto}
        "#,
            formato = formato,
            texto = texto_legal
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
