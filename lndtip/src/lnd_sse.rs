use futures::StreamExt;
use futures::executor::{block_on};
use std::convert::Infallible;
use warp::{sse::ServerSentEvent, Stream};

use crate::lnd_service;

/*
pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent + Send + 'static, Infallible>> + Send + 'static {
    let r_hash : String = check.r_hash.unwrap().to_owned();
    let mut stream = block_on(ls.status_stream(&r_hash));
    stream.map(move |status| {
        Ok(warp::sse::json(status.clone()))
    })
}
*/
