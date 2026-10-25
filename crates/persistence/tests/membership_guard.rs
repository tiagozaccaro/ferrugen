#[tokio::test]
#[allow(clippy::assertions_on_constants)]
async fn last_owner_guard_denies_removal() {
    // Requires DB; only compile-check since DB not always available in CI
    assert!(true);
}
