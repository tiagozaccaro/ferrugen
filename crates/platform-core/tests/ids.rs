use platform_core::{SessionId, TenantId, UserId};

#[test]
fn generated_ids_are_distinct() {
    assert_ne!(UserId::new(), UserId::new());
    assert_ne!(TenantId::new(), TenantId::new());
    assert_ne!(SessionId::new(), SessionId::new());
}
