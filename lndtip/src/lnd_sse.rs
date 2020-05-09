use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Filter};

use crate::lnd_service;

fn sse_invoicestatus(status: lnd_service::InvoiceStatus) -> Result<impl ServerEvent, Infallible> {
    Ok(warp::sse::json(status))
}

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    let event_stream = interval(
        Duration::from_secs(2)).map(move |_| {
            let status = ls.lookup(&check.r_hash.unwrap_or_default());
            Ok(warp::sse::json(&status))
        });
    Ok(warp::sse::reply(event_stream))
}
    
