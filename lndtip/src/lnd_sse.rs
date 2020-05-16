use std::convert::Infallible;
use std::time::Duration;
use futures::StreamExt;
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Stream};

use crate::lnd_service;

pub fn invoice_poll(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    interval(Duration::from_secs(2)).map(move |_| {
        let status = ls.lookup(check.r_hash.as_ref().unwrap());
        Ok(warp::sse::json(status.clone()))
    })
}

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    let r_hash = check.r_hash.as_ref().unwrap().as_bytes().to_vec();
    let stream = ls.invoice_stream(r_hash).unwrap();
    stream.map(move |status| Ok(warp::sse::json(status.clone())))
}
