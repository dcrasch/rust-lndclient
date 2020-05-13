use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;

use grpc::ClientStub;
use grpc::MetadataKey;

use anyhow::Error;
use tls_api::TlsConnector;
use tls_api::TlsConnectorBuilder;

use futures::executor;
use futures::stream::Stream;

use crate::rpc;
use crate::rpc_grpc::{LightningClient, WalletUnlockerClient};

pub struct ClientBuilder {
    address: String,
    certificate: String,
    macaroon: String,
}

impl ClientBuilder {
    pub fn build(self) -> Result<LndClient, Error> {
        let client = LndClient::new(self.address, self.certificate, self.macaroon)?;
        Ok(client)
    }
}

pub struct LndClient {
    pub macaroon: String,
    pub lightningclient: Arc<LightningClient>,
    pub walletunlockerclient: Arc<WalletUnlockerClient>,
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

    pub fn new(address: String, certificate: String, macaroon: String) -> Result<Self, Error> {
        let socket_addr: SocketAddr = address.parse().unwrap();
        let host = socket_addr.ip().to_string();
        let port = socket_addr.port();

        let root_ca = tls_api::Certificate {
            bytes: certificate.as_bytes().to_vec(),
            format: tls_api::CertificateFormat::PEM,
        };

        let mut builder = tls_api_native_tls::TlsConnector::builder()?;
        builder.add_root_certificate(root_ca)?;
        let connector = builder.build()?;

        let tls_option = httpbis::ClientTlsOption::Tls(host.to_owned(), Arc::new(connector));

        let grpc_client = Arc::new(
            grpc::ClientBuilder::new(&host, port)
                .explicit_tls(tls_option)
                .build()?,
        );

        let lightningclient = Arc::new(LightningClient::with_client(grpc_client.clone()));
        let walletunlockerclient = Arc::new(WalletUnlockerClient::with_client(grpc_client.clone()));
        Ok(Self {
            macaroon,
            lightningclient,
            walletunlockerclient,
        })
    }

    fn request_options(&self) -> grpc::RequestOptions {
        let mut options = grpc::RequestOptions::new();
        options.metadata.add(
            MetadataKey::from("macaroon"),
            self.macaroon.as_bytes().to_vec().into(),
        );
        options.cachable = true;
        options
    }

    pub fn get_info(&self) -> Result<rpc::GetInfoResponse, Error> {
        let options = self.request_options();
        let req = rpc::GetInfoRequest::new();
        let resp = self.lightningclient.get_info(options, req).drop_metadata();
        Ok(executor::block_on(resp)?)
    }

    pub fn get_wallet_balance(&self) -> Result<rpc::WalletBalanceResponse, Error> {
        let options = self.request_options();
        let req = rpc::WalletBalanceRequest::new();
        let resp = self
            .lightningclient
            .wallet_balance(options, req)
            .drop_metadata();
        Ok(executor::block_on(resp)?)
    }

    pub fn lookup_invoice(&self, r_hash: &[u8]) -> Result<rpc::Invoice, Error> {
        let options = self.request_options();
        let mut req = rpc::PaymentHash::new();
        req.r_hash = r_hash.into();
        let resp = self
            .lightningclient
            .lookup_invoice(options, req)
            .drop_metadata();
        Ok(executor::block_on(resp)?)
    }

    pub fn list_invoice(
        &self,
        pending_only: bool,
        index_offset: u64,
        num_max_invoices: u64,
        reversed: bool,
    ) -> Result<rpc::ListInvoiceResponse, Error> {
        let options = self.request_options();
        let req = rpc::ListInvoiceRequest::new();
        let resp = self
            .lightningclient
            .list_invoices(options, req)
            .drop_metadata();
        Ok(executor::block_on(resp)?)
    }

    pub fn add_invoice(
        &self,
        satoshi: i64,
        memo: &str,
        expiry: i64,
    ) -> Result<rpc::AddInvoiceResponse, Error> {
        let options = self.request_options();
        let mut req = rpc::Invoice::new();
        req.value = satoshi;
        req.memo = memo.to_string();
        req.expiry = expiry;
        let resp = self
            .lightningclient
            .add_invoice(options, req)
            .drop_metadata();
        Ok(executor::block_on(resp)?)
    }

    pub fn subscribe_invoices(
        &self,
        add_index: u64,
        settle_index: u64,
    ) -> Result<dyn Stream<Item = Result<rpc::Invoice, Error>> + Send + 'static, Error> {
        let options = self.request_options();
        let mut req = rpc::InvoiceSubscription::new();
        let resp = self
            .lightningclient
            .subscribe_invoices(options, req)
            .drop_metadata();
        Ok(executor::block_on(resp)?)
    }
}
