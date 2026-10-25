use application::audit::{AuditError, AuditEvent, AuditSink};
use std::sync::Mutex;

pub struct TracingAuditSink;

#[async_trait::async_trait]
impl AuditSink for TracingAuditSink {
    async fn record(&self, event: AuditEvent) -> Result<(), AuditError> {
        tracing::info!(?event, "audit event");
        Ok(())
    }
}

pub struct RecordingAuditSink {
    events: Mutex<Vec<AuditEvent>>,
}

impl RecordingAuditSink {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().unwrap().clone()
    }
}

impl Default for RecordingAuditSink {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AuditSink for RecordingAuditSink {
    async fn record(&self, event: AuditEvent) -> Result<(), AuditError> {
        self.events.lock().unwrap().push(event);
        Ok(())
    }
}
