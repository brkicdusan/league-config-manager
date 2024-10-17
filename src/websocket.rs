use std::sync::{Arc, Mutex, MutexGuard};

use iced::{
    futures::{channel::mpsc::Sender, SinkExt, Stream},
    stream,
};
use league_client::client;
use serde_json::{Number, Value};
use tokio::runtime;

#[derive(Debug, Clone)]
pub(crate) enum Event {
    Selected(i32),
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
            // let _ = output.send(Event::Selected(20)).await;

            let output = Arc::clone(&output);
            let mut sender = output.lock().unwrap().to_owned();

            rt.spawn(async move {
                let _res = lcu(sender).await;
            })
            .await
            .unwrap();

            // TODO: retry message preko ovoga i povecaj delay
            println!("retrying...");
            rt.spawn(async {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
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
    let mut cnt = 1;
    let _ = output.send(Event::Selected(cnt)).await;
    while let Ok(msg) = speaker.reader.recv_async().await {
        let msg = msg.into_message();
        if msg.uri == "/lol-champ-select/v1/current-champion" {
            let x=  match &msg.data {
                Value::Number(num) => num.as_i64(),
                _ => Some(-5)
            }.unwrap();
            println!("{:?}", msg.data);
            let _ = output.send(Event::Selected(x as i32)).await;
            cnt += 1;
        }
    }
    Ok(())
}
