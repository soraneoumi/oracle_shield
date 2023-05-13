use tokio::time::{sleep, Duration};

pub async fn calculate_pi(n: u64) -> f64 {
    let mut pi = 0.0;
    let mut sign = 1.0;

    for k in 0..n {
        pi += sign * (4.0 / (8.0 * k as f64 + 1.0) - 2.0 / (8.0 * k as f64 + 4.0) - 1.0 / (8.0 * k as f64 + 5.0) - 1.0 / (8.0 * k as f64 + 6.0)) / (16.0_f64).powf(k as f64);
        sign *= -1.0;
        sleep(Duration::from_millis(100)).await;
    }

    pi
}
