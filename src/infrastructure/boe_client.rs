use chrono::NaiveDate;
use reqwest::Client;
use anyhow::{Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use regex::Regex;
use serde::de::value;
use crate::model::articulo::Articulo;
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

        self.parse_articulo(&response).await?;
        
        Ok(response)
    }

    pub async fn parse_articulo(&self, xml: &str) -> Result<String> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        let mut buf = Vec::new();
        //let mut bloque_id = String::new();

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
                    print!("id: {:?}, tipo: {:?}, titulo: {:?}", id, tipo, titulo)

                }
                Ok(Event::Start(e)) if e.name().as_ref() == b"version" => {
                    let mut fecha_vigencia = None;
                    //let mut id_norma = None;
                    for attr in e.attributes().flatten(){
                        match attr.key.as_ref() {
                            b"fecha_vigencia" => {
                                let  value = attr.unescape_value().unwrap();
                                fecha_vigencia = Some(NaiveDate::parse_from_str(&value, "%Y%m%d").unwrap());
                                println!("fecha: {:?}", fecha_vigencia)
                            },
                            _=>{}
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error: {:?}", e),
                _ => {}
            }
        }

        Ok("31".to_string())
    }
}