use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::sync::Mutex;

use crate::{AuthError, LoginThrottle};

#[derive(Debug, Clone)]
struct ThrottleState {
    failures: u32,
    window_start: DateTime<Utc>,
    locked_until: Option<DateTime<Utc>>,
}

pub struct InMemoryLoginThrottle {
    max_attempts: u32,
    window: Duration,
    lockout: Duration,
    state: Mutex<HashMap<String, ThrottleState>>,
}

impl InMemoryLoginThrottle {
    pub fn new(max_attempts: u32, window: Duration, lockout: Duration) -> Self {
        Self {
            max_attempts,
            window,
            lockout,
            state: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl LoginThrottle for InMemoryLoginThrottle {
    async fn check_and_record(
        &self,
        normalized_email: &str,
        now: DateTime<Utc>,
    ) -> Result<(), AuthError> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let entry = state
            .entry(normalized_email.to_string())
            .or_insert(ThrottleState {
                failures: 0,
                window_start: now,
                locked_until: None,
            });

        if let Some(locked_until) = entry.locked_until {
            if now < locked_until {
                return Err(AuthError::TooManyAttempts);
            } else {
                entry.locked_until = None;
                entry.failures = 0;
                entry.window_start = now;
            }
        }

        if now - entry.window_start > self.window {
            entry.failures = 0;
            entry.window_start = now;
        }

        entry.failures += 1;
        if entry.failures >= self.max_attempts {
            entry.locked_until = Some(now + self.lockout);
        }
        if entry.failures >= self.max_attempts {
            Err(AuthError::TooManyAttempts)
        } else {
            Ok(())
        }
    }

    async fn clear(&self, normalized_email: &str) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.remove(normalized_email);
    }
}
