use crate::lnd_service;
use futures::{Stream, StreamExt};
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use warp::sse::ServerSentEvent;

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    let r_hash: String = check.r_hash.unwrap();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            let status = ls.lookup(&r_hash).await;
            if tx.send(status).is_err() {
		break;
	    }
	    tokio::time::delay_for(Duration::from_secs(2)).await;
        }
    });
    rx.map(|status| Ok(warp::sse::json(status)))
}
