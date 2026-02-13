use anyhow::Result;

mod infrastructure;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting BOE sync batch...");

    let boe_client = infrastructure::boe_client::BoeClient::new();

    let xml = boe_client.fetch_codigo_penal_xml().await?;

    println!("Downloaded XML size: {}", xml.len());

    // Aquí luego parseamos y sincronizamos DB

    Ok(())
}
