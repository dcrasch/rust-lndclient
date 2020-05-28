use crate::lnd_handlers;
use crate::lnd_service;
use crate::lnd_sse;

use warp::Filter;

pub fn invoices(
    ld: lnd_service::LightningService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    invoice_create(ld.clone())
        .or(invoice_check(ld.clone()))
  //      .or(invoice_watch(ld.clone()))
}


pub fn invoice_create(
    ls: lnd_service::LightningService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("createinvoice")
        .and(warp::post())
        .and(json_body())
        //.and(warp::query::<lnd_service::InvoiceRequest>())
        .and(with_ls(ls))
        .and_then(lnd_handlers::create_invoice)
}

pub fn invoice_check(
    ls: lnd_service::LightningService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("checkinvoice")
        .and(warp::post())
        .and(json_body2())
        .and(with_ls(ls))
        .and_then(lnd_handlers::status_invoice)
}

/*
pub fn invoice_watch(
    ls: lnd_service::LightningService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("watchinvoice")
        .and(warp::get())
        .and(warp::query::<lnd_service::CheckOptions>())
        .and(with_ls(ls))
        .map(move |c, l| {
            warp::sse::reply(warp::sse::keep_alive().stream(lnd_sse::invoice_events(c, l)))
        })
}
*/

fn with_ls(
    ls: lnd_service::LightningService,
) -> impl Filter<Extract = (lnd_service::LightningService,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || ls.clone())
}

fn json_body(
) -> impl Filter<Extract = (lnd_service::InvoiceRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn json_body2(
) -> impl Filter<Extract = (lnd_service::CheckOptions,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
