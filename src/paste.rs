use std::sync::Arc;

use tokio::{runtime, sync::Mutex};

pub(crate) async fn post(
    client: Arc<Mutex<reqwest::Client>>,
    content: String,
) -> Result<String, reqwest::Error> {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let params = [("content", content)];

    let res = rt
        .spawn(async move {
            let client = client.lock().await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            client
                .post("https://dpaste.com/api/")
                .header(
                    "User-Agent",
                    "League Config Manager - Settings manager for League of Legends",
                )
                .form(&params)
                .send()
                .await
        })
        .await
        .unwrap()?;

    res.error_for_status_ref()?;
    let link = res.headers()["location"].to_str().unwrap().to_string();
    Ok(link)
}

pub(crate) async fn get(
    client: Arc<Mutex<reqwest::Client>>,
    link: String,
) -> Result<String, reqwest::Error> {
    let rt = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let res = rt
        .spawn(async move {
            let client = client.lock().await;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

            client
                .get(format!("{link}.txt"))
                .header(
                    "User-Agent",
                    "League Config Manager - Settings manager for League of Legends",
                )
                .send()
                .await
        })
        .await
        .unwrap()?;

    res.error_for_status_ref()?;

    res.text().await
}
