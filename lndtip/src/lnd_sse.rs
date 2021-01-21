use crate::lnd_service;
use futures::{Stream, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc;
use warp::sse::Event;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub fn invoice_events(
    check: lnd_service::CheckOptions,
    ls: lnd_service::LightningService,
) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    let r_hash: String = check.r_hash.unwrap();

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    
    tokio::spawn(async move {
        loop {
            let status = ls.lookup(&r_hash).await;
            if tx.send(status).is_err() {
		break;
	    }
	    tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
    rx.map(|status| Ok(Event::default().json_data(status).unwrap()))
}
