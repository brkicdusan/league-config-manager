use iced::{
    futures::{channel::mpsc::Sender, SinkExt, Stream},
    stream,
};
use league_client::client;
use tokio::runtime;

#[derive(Debug, Clone)]
pub(crate) enum Event {
    Selected(i32),
}

pub fn connect() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        loop {
            let _res = lcu(&mut output).await;

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

async fn lcu(output: &mut Sender<Event>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let builder = client::Client::builder()?;
    let lc = builder.insecure(true).build()?;

    let connected = lc.connect_to_socket().await?;

    let speaker = league_client::subscribe(connected).await;

    let msg = (5, "OnJsonApiEvent");
    let msg = serde_json::to_string(&msg).unwrap();

    speaker.send(msg).await.expect("should have sent a message");

    while let Ok(msg) = speaker.reader.recv_async().await {
        println!("{:?}", msg.into_message());
        let _ = output.send(Event::Selected(10)).await;
    }
    Ok(())
}
