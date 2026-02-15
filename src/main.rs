use std::env;

use anyhow::Result;
use dotenvy::dotenv;

mod infrastructure;
mod model;

#[tokio::main]
async fn main() -> Result<()> {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database URL is missing in the config file");
    let _connection = infrastructure::db::create_pool(&database_url).await?;


    println!("Starting BOE sync batch...");

    let boe_client = infrastructure::boe_client::BoeClient::new();

    let xml = boe_client.fetch_codigo_penal_xml().await?;

    println!("Downloaded XML size: {}", xml.len());

    Ok(())
}
