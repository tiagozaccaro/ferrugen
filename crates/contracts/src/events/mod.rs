pub trait Event: Send + Sync { fn event_type(&self) -> &'static str; }
Contracts define event interfaces only (import control).
Domain/app provide concrete implementations.
No separate events crate — contracts/domain/app split enforced.
