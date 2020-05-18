use rust_lndclient::client::LndClient;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub struct LndConfig {
    host: String,
    cert: String,
    macaroon: String,
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filecontents = fs::read_to_string("lndtip.toml").expect("Error lndtip.toml");
    let lndconfig: LndConfig = toml::from_str(&filecontents).expect("Error parsing config");

    let mut client = LndClient::builder(lndconfig.host, lndconfig.cert, lndconfig.macaroon)?
        .build()
        .await?;
    if let Ok(invoices) = client.list_invoice(false, 9, 8888888, false).await {
        if let Some(last_invoice) = invoices.invoices.last() {
            if let Ok(invoice) = client.lookup_invoice(last_invoice.r_hash.as_slice()).await {
                print!("{:?}", invoice);
            }
        }
    }

    Ok(())
}
