use futures::StreamExt;
use futures::executor::{block_on};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Stream};

use crate::lnd_service;

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    interval(Duration::from_secs(2)).map(move |_| {
        let status = block_on(ls.lookup(check.r_hash.as_ref().unwrap()));
        Ok(warp::sse::json(status.clone()))
    })
}
