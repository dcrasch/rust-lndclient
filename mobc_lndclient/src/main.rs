use anyhow::Error;
use serde::Deserialize;
use std::fs;
use mobc_lndclient::{LightningConnectionInfo,LightningConnectionManager};
use mobc_lndclient::mobc::{Pool};

#[derive(Deserialize)]
pub struct LndConfig {
    host: String,
    cert: String,
    macaroon: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let filecontents = fs::read_to_string("lndtip.toml").expect("Error lndtip.toml");
    let config: LndConfig = toml::from_str(&filecontents).expect("Error parsing config");

    
    let manager = LightningConnectionManager::new(LightningConnectionInfo {
        host: config.host,
        cert: config.cert,
        macaroon: config.macaroon,
    })
    .unwrap();

    let pool = Pool::builder().build(manager);

    let mut conn = pool.get().await.unwrap();

    let my_hash : Vec<u8> = vec![67, 59, 206, 149, 242, 61, 56, 222, 141, 110, 52, 88, 214, 170, 222, 64, 33, 217, 81, 201, 56, 247, 255, 190, 2, 242, 74, 163, 214, 68, 110, 198];
    
    if let Ok(invoices) = conn.list_invoices(false, 9, 888888, false).await {
        if invoices.invoices.last().is_some() {
	    let r_hash = my_hash.as_slice();
	    println!("rhash: {:?}",r_hash);
            if let Ok(invoice) = conn.lookup_invoice(r_hash).await {
                print!("{:?}", invoice);
            }
	    else {
		println!("notfound");
	    }
        }
    }

    Ok(())
}
