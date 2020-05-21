use crate::lnd_service;

use std::convert::Infallible;

pub async fn create_invoice(
    create: lnd_service::InvoiceRequest,
    ls: lnd_service::LightningService,
) -> Result<impl warp::Reply, Infallible> {
    let invoice = ls.add_invoice(create.satoshi, &create.description, 3600).await;
    Ok(warp::reply::json(&invoice))
}

pub async fn _invoice(
    create: lnd_service::InvoiceRequest,
    ls: lnd_service::LightningService,
) -> Result<impl warp::Reply, Infallible> {
    let invoice = ls.add_invoice(create.satoshi, &create.description, 3600).await;
    Ok(warp::reply::json(&invoice))
}

pub async fn status_invoice(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> Result<impl warp::Reply, Infallible> {
    let status = ls.lookup(&check.r_hash.unwrap_or_default()).await;
    Ok(warp::reply::json(&status))
}
