use std::net::SocketAddr;

use anyhow::Error;

use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Request;

use crate::lnrpc;
use crate::lnrpc::lightning_client::LightningClient;
use crate::lnrpc::wallet_unlocker_client::WalletUnlockerClient;

pub struct ClientBuilder {
    address: String,
    certificate: String,
    macaroon: String,
}

impl ClientBuilder {
    pub async fn build(self) -> Result<LndClient, Error> {
        let client = LndClient::new(self.address, self.certificate, self.macaroon).await?;
        Ok(client)
    }
}

pub struct LndClient {
    pub macaroon: String,
    pub lightningclient: LightningClient<tonic::transport::Channel>,
    pub walletunlockerclient: WalletUnlockerClient<tonic::transport::Channel>,
}

impl LndClient {
    pub fn builder(
        address: String,
        certificate: String,
        macaroon: String,
    ) -> Result<ClientBuilder, Error> {
        Ok(ClientBuilder {
            address: address,
            certificate: certificate,
            macaroon: macaroon,
        })
    }

    // mmmm async new bad?
    pub async fn new(
        address: String,
        certificate: String,
        macaroon: String,
    ) -> Result<Self, Error> {
        let socket_addr: SocketAddr = address.parse().unwrap();
        let host = socket_addr.ip().to_string();
        let port = socket_addr.port();

        let ca = Certificate::from_pem(&certificate);
        let tls = ClientTlsConfig::new()
            .ca_certificate(ca)
            .domain_name("localhost"); // self signed
        let channel = Channel::from_shared(format!("http://{}:{}", host, port))?
            .tls_config(tls)
            .connect()
            .await?;

        let token = MetadataValue::from_str(&macaroon)?;

        let lightningclient =
            LightningClient::with_interceptor(channel.clone(), move |mut req: Request<()>| {
                req.metadata_mut().insert("macaroon", token.clone());
                Ok(req)
            });
        let walletunlockerclient = WalletUnlockerClient::new(channel);
        Ok(Self {
            macaroon,
            lightningclient,
            walletunlockerclient,
        })
    }

    pub async fn get_info(&mut self) -> Result<lnrpc::GetInfoResponse, Error> {
        let req = tonic::Request::new(lnrpc::GetInfoRequest {});
        let resp = self.lightningclient.get_info(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn get_wallet_balance(&mut self) -> Result<lnrpc::WalletBalanceResponse, Error> {
        let req = tonic::Request::new(lnrpc::WalletBalanceRequest::default());
        let resp = self.lightningclient.wallet_balance(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn lookup_invoice(&mut self, r_hash: &[u8]) -> Result<lnrpc::Invoice, Error> {
        let req = tonic::Request::new(lnrpc::PaymentHash {
            r_hash: r_hash.into(),
            ..lnrpc::PaymentHash::default()
        });
        let resp = self.lightningclient.lookup_invoice(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn list_invoice(
        &mut self,
        pending_only: bool,
        index_offset: u64,
        num_max_invoices: u64,
        reversed: bool,
    ) -> Result<lnrpc::ListInvoiceResponse, Error> {
        let req = tonic::Request::new(lnrpc::ListInvoiceRequest::default());
        let resp = self.lightningclient.list_invoices(req).await?;
        Ok(resp.into_inner())
    }

    pub async fn add_invoice(
        &mut self,
        satoshi: i64,
        memo: &str,
        expiry: i64,
    ) -> Result<lnrpc::AddInvoiceResponse, Error> {
        let req = tonic::Request::new(lnrpc::Invoice {
            value: satoshi,
            memo: memo.to_string(),
            expiry: expiry,
            ..lnrpc::Invoice::default()
        });
        let resp = self.lightningclient.add_invoice(req).await?;
        Ok(resp.into_inner())
    }
}
