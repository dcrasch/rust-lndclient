pub extern crate mobc;

use mobc::{Manager, async_trait};
use rust_lndclient::client::LndClient;


#[derive(Debug)]
pub struct LightningConnectionInfo {
    pub host: String,
    pub cert: String,
    pub macaroon: String,
}

#[derive(Debug)]
pub struct LightningConnectionManager {
    connection_info: LightningConnectionInfo,
}

impl LightningConnectionManager {
    pub fn new(params: LightningConnectionInfo) -> Result<LightningConnectionManager, anyhow::Error> {
        Ok(LightningConnectionManager {
            connection_info: params,
        })
    }
}

#[async_trait]
impl Manager for LightningConnectionManager {
   type Connection = LndClient;
   type Error = anyhow::Error;

   async fn connect(&self) -> Result<Self::Connection, Self::Error> {
       eprintln!("build new connection mobc");
       let client = LndClient::builder(
            self.connection_info.host.clone(),
            self.connection_info.cert.clone(),
            self.connection_info.macaroon.clone(),
        )?;
       let res = client.build().await?;
       Ok(res)
   }

   async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
       Ok(conn)
   }
}
