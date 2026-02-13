use reqwest::Client;

pub struct BoeClient {
    client: Client,
}

impl BoeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_codigo_penal_xml(&self) -> anyhow::Result<String>{
        let url = "https://www.boe.es/datosabiertos/api/legislacion-consolidada/id/BOE-A-1995-25444/texto";

        let response = self.client
            .get(url)
            .header("Accept", "application/xml")
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
