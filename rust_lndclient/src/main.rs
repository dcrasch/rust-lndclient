use futures::executor::block_on;
use futures::{Stream, StreamExt};
use std::time::Duration;
use tokio::time::interval;

use rust_lndclient::client::LndClient;
use serde::Deserialize;
use std::fs;

#[derive(Clone, Debug)]
pub struct InvoiceStatus {
    pub status: String,
    pub settled: bool,
    pub expiry: i64,
}

#[derive(Deserialize)]
pub struct LndConfig {
    host: String,
    cert: String,
    macaroon: String,
}

pub fn invoice_events(
    r_hash: &[u8],
    mut client: LndClient,
) -> impl Stream<Item = InvoiceStatus> + '_ {
    interval(Duration::from_secs(2)).map(move |_| {
        if let Ok(invoice) = block_on(client.lookup_invoice(r_hash)) {
            InvoiceStatus {
                status: format!("{:?}", invoice.state),
                settled: invoice.settled,
                expiry: invoice.expiry,
            }
        } else {
            InvoiceStatus {
                status: "notfound".to_string(),
                settled: false,
                expiry: 0,
            }
        }
    })
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filecontents = fs::read_to_string("lndtip.toml").expect("Error lndtip.toml");
    let lndconfig: LndConfig = toml::from_str(&filecontents).expect("Error parsing config");

    let mut client = LndClient::builder(lndconfig.host, lndconfig.cert, lndconfig.macaroon)?
        .build()
        .await?;
    if let Ok(invoices) = client.list_invoices(false, 9, 8888888, false).await {
        if let Some(last_invoice) = invoices.invoices.last() {
            let mut stream = invoice_events(last_invoice.r_hash.as_ref(), client);
            while let Some(e) = stream.next().await {
                println!("{:?}", e);
            }
        }
    }

    Ok(())
}
