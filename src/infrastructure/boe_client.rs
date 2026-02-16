use chrono::NaiveDate;
use reqwest::Client;
use anyhow::{Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use super::openai_client::OpenAIClient;

pub struct BoeClient {
    client: Client,
}

impl BoeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_codigo_penal_xml(&self) -> Result<String>{
        let url = "https://www.boe.es/datosabiertos/api/legislacion-consolidada/id/BOE-A-1995-25444/texto/bloque/a234";

        let response = self.client
            .get(url)
            .header("Accept", "application/xml")
            .send()
            .await?
            .text()
            .await?;

        parse_articulo(&response).await?;
        
        Ok(response)
    }
}


async fn parse_articulo(xml: &str) -> Result<String> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        //let mut bloque_id = String::new();
        let mut id_norma = None;
        let mut texto_articulo = String::new();

        let mut dentro_parrafo_valido = false;
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) if e.name().as_ref() == b"bloque" => {
                    let mut id = None;
                    let mut tipo = None;
                    let mut titulo = None;
                    for attr in e.attributes().flatten(){
                        match attr.key.as_ref() {
                            b"id" => id = Some(attr.unescape_value().unwrap().to_string()),
                            b"tipo" => tipo = Some(attr.unescape_value().unwrap().to_string()),
                            b"titulo" => titulo = Some(attr.unescape_value().unwrap().to_string()),
                            _ => {}
                        }
                    }

                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"version" => {
                    for attr in e.attributes().flatten(){
                        match attr.key.as_ref() {
                            b"id_norma" => {
                                texto_articulo = String::new();
                                id_norma = Some(attr.unescape_value().unwrap().to_string());
                            }
                            _=>{}
                        }
                    }
                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"p" => {

                    let mut clase = None;

                    for attr in e.attributes().flatten() {
                        if attr.key.as_ref() == b"class" {
                            clase = Some(attr.unescape_value().unwrap().to_string());
                        }
                    }

                    if let Some(c) = clase {
                        if c == "parrafo" {
                            dentro_parrafo_valido = true;
                        }
                    }
                }   

                Ok(Event::Text(e)) if dentro_parrafo_valido => {
                    texto_articulo.push_str(&e.unescape().unwrap());
                    texto_articulo.push('\n');
                }

                Ok(Event::End(e)) if e.name().as_ref() == b"p" => {
                    dentro_parrafo_valido = false;
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error: {:?}", e),
                _ => {}
            }
        }
        println!("{:?}", texto_articulo);
        let client = OpenAIClient::new();
        
         match client.chat(&texto_articulo).await {
            Ok(response) => println!("Respuesta: {}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
         
       


        Ok("".to_string())
    }