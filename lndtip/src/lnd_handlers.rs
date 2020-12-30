use crate::lnd_service;

use std::convert::Infallible;

pub async fn create_invoice(
    create: lnd_service::InvoiceRequest,
    ls: lnd_service::LightningService,
) -> Result<impl warp::Reply, Infallible> {
    let invoice = ls
        .add_invoice(create.satoshi, &create.description, 3600)
        .await;
    Ok(warp::reply::json(&invoice))
}
