use iced::{
    futures::{SinkExt, Stream},
    stream,
};
use tokio::runtime;

#[derive(Debug, Clone)]
pub(crate) enum Event {
    Selected(i32),
}

pub fn connect() -> impl Stream<Item = Event> {
    stream::channel(100, |mut output| async move {
        let mut cnt = 0;
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        loop {
            rt.spawn(async {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            })
            .await
            .unwrap();

            let _ = output.send(Event::Selected(cnt)).await;
            println!("sent");

            cnt += 1;
        }
    })
}
