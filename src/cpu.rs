use tokio::time::{sleep, Duration};

pub async fn calculate_pi() {
    let handle = tokio::spawn(async {
        loop {
        }
    });
    sleep(Duration::from_secs(300)).await;
    handle.await.unwrap();
}
