use tokio::time::{sleep, Duration};
use rand::rngs::OsRng;
use rand::RngCore;

pub async fn calculate_pi() {
    let handle = tokio::spawn(async {
        let mut rng = OsRng;

        let mut total_points = 0;
        let mut inside_circle = 0;

        loop {
            let mut bytes = [0; 8];
            rng.fill_bytes(&mut bytes);

            let x: f64 = f64::from_le_bytes(bytes[0..4].try_into().unwrap()) * 2.0 - 1.0;
            let y: f64 = f64::from_le_bytes(bytes[4..8].try_into().unwrap()) * 2.0 - 1.0;

            total_points += 1;
            if x * x + y * y <= 1.0 {
                inside_circle += 1;
            }

            let _pi_approximation = 4.0 * (inside_circle as f64) / (total_points as f64);

            // Adjust sleep duration based on desired accuracy
            sleep(Duration::from_secs(1)).await;
        }
    });

    sleep(Duration::from_secs(300)).await;
    handle.await.unwrap();
}
