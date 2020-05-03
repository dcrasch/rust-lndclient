use base64;
use r2d2_lndclient::lnd_pool::LightningConnectionManager;
use r2d2_lndclient::r2d2::Pool;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct LightningService {
    lnc: Pool<LightningConnectionManager>,
}

impl LightningService {
    pub fn new(lnc: Pool<LightningConnectionManager>) -> Self {
        Self { lnc: lnc }
    }

    pub fn add_invoice(&self, satoshi: i64, description: &str, expiry: i64) -> InvoiceResponse {
        let conn = self.lnc.get().unwrap();
        if let Ok(invoice_response) = conn.add_invoice(satoshi, description, expiry) {
            if let Ok(invoice) = conn.lookup_invoice(invoice_response.r_hash.as_slice()) {
                let r_hash = base64::encode(invoice.r_hash);
                return InvoiceResponse {
                    r_hash: r_hash,
                    expiry: invoice.expiry,
                    bolt11: invoice.payment_request,
                };
            }
        }
        InvoiceResponse {
            r_hash: "".to_string(),
            expiry: 0,
            bolt11: "".to_string(),
        }
    }

    pub fn lookup(&self, r_hash: &str) -> InvoiceStatus {
        let conn = self.lnc.get().unwrap();
        if let Ok(r_hash) = base64::decode(r_hash) {
            if let Ok(invoice) = conn.lookup_invoice(r_hash.as_slice()) {
                return InvoiceStatus {
                    status: format!("{:?}", invoice.state),
                    settled: invoice.settled,
                    expiry: invoice.expiry,
                };
            }
        }
        InvoiceStatus {
            status: "notfound".to_string(),
            expiry: 0,
            settled: false,
        }
    }
}

#[derive(Deserialize)]
pub struct InvoiceRequest {
    pub satoshi: i64,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub r_hash: String,
    pub expiry: i64,
    pub bolt11: String,
}

#[derive(Deserialize)]
pub struct CheckOptions {
    pub r_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct InvoiceStatus {
    pub status: String,
    pub settled: bool,
    pub expiry: i64,
}

#[derive(Deserialize)]
pub struct InvoiceError {
    pub error: String,
}
