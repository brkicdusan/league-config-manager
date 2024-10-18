use std::sync::{Arc, Mutex};

use iced::{
    futures::{channel::mpsc::Sender, SinkExt, Stream},
    stream,
};
use league_client::client;
use serde_json::Value;
use tokio::runtime;

#[derive(Debug, Clone)]
pub(crate) enum Event {
    Selected(u32),
    Connected,
    Disconnected,
    Retrying(u32),
}

pub fn connect() -> impl Stream<Item = Event> {
    println!("test print");
    stream::channel(100, |mut output| async move {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let output = Arc::new(Mutex::new(output));

        loop {
            let output = Arc::clone(&output);
            let mut sender = output.lock().unwrap().to_owned();

            rt.spawn(async move {
                let _ = lcu(sender).await;
            })
            .await
            .unwrap();

            let output = Arc::clone(&output);
            let mut sender = output.lock().unwrap().to_owned();

            let _ = sender.send(Event::Disconnected).await;

            rt.spawn(async move {
                let delay = 10;
                for i in (1..=delay).rev() {
                    let _ = sender.send(Event::Retrying(i)).await;
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            })
            .await
            .unwrap();
        }
    })
}

async fn lcu(mut output: Sender<Event>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let builder = client::Client::builder()?;
    let lc = builder.insecure(true).build()?;

    let connected = lc.connect_to_socket().await?;

    let speaker = league_client::subscribe(connected).await;

    let msg = (5, "OnJsonApiEvent");
    let msg = serde_json::to_string(&msg).unwrap();

    speaker.send(msg).await.expect("should have sent a message");
    let _ = output.send(Event::Connected).await;
    while let Ok(msg) = speaker.reader.recv_async().await {
        let msg = msg.into_message();
        if msg.uri == "/lol-champ-select/v1/current-champion" {
            let x = match &msg.data {
                Value::Number(num) => num.as_u64(),
                _ => Some(0),
            }
            .unwrap();
            let _ = output.send(Event::Selected(x as u32)).await;
        }
    }
    Ok(())
}
