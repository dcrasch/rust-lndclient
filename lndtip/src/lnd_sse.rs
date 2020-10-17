use crate::lnd_service;
use futures::executor::block_on;
use futures::{Stream, StreamExt};
use std::convert::Infallible;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
use warp::{sse::ServerSentEvent, Filter};

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    let r_hash: String = check.r_hash.unwrap().to_owned();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            let status = ls.lookup(&r_hash).await;
            tx.send(status);
            tokio::time::delay_for(Duration::from_secs(2)).await;
            println!("look up");
        }
    });
    rx.map(|status| Ok(warp::sse::json(status.clone())))
}
