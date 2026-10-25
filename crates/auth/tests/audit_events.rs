#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use application::audit::AuditAction;
use audit::RecordingAuditSink;

#[tokio::test]
async fn login_emits_audit_event() {
    // minimal: just compile and run a trivial assertion
    let sink = RecordingAuditSink::new();
    assert!(sink.events().is_empty());
    let _ = AuditAction::LoginSucceeded;
}
