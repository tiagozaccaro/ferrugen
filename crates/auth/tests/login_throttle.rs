#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use auth::InMemoryLoginThrottle;
use auth::LoginThrottle;
use chrono::Utc;

#[tokio::test]
async fn lockout_after_five_failures() {
    let throttle = InMemoryLoginThrottle::new(
        5,
        chrono::Duration::minutes(15),
        chrono::Duration::minutes(15),
    );
    let email = "a@example.com";
    for _ in 0..5 {
        let _ = throttle.check_and_record(email, Utc::now()).await;
    }
    let res = throttle.check_and_record(email, Utc::now()).await;
    assert!(res.is_err());
}
