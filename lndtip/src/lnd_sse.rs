use futures::StreamExt;
use futures::executor::{block_on};
use std::convert::Infallible;
use std::time::Duration;
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Stream};

use crate::lnd_service;

pub fn status_stream<'a,'b>(
    grrr_hash: String,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = lnd_service::InvoiceStatus> + 'b {
    interval(Duration::from_secs(2)).map(move |_| {
        block_on(ls.lookup(grrr_hash.as_ref()))
    })
}

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent + Send + 'static, Infallible>> + Send + 'static {
    let r_hash : String = check.r_hash.unwrap().to_owned();
    let mut stream = status_stream("GhDr/yOvDNH11ArQN/yDHWeP6mdwHD7H7V7PH2VqgVY=".to_string(),ls);
    stream.map(move |status| {
        Ok(warp::sse::json(status.clone()))
    })
}
