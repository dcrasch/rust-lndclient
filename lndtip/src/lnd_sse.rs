use futures::StreamExt;
use futures::executor::{block_on};
use std::convert::Infallible;
use warp::{sse::ServerSentEvent, Stream};
use tokio::time::interval;
use crate::lnd_service;
use std::time::Duration;

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    let r_hash : String = check.r_hash.unwrap().to_owned();
    block_on(async move {
        interval(Duration::from_secs(2)).map(move |_| {
        block_on(ls.lookup(r_hash.as_ref()))
    })
    })
    .map(|status|        Ok(warp::sse::json(status.clone())))

    https://users.rust-lang.org/t/tokio-interval-not-working-in-runtime/41260
}