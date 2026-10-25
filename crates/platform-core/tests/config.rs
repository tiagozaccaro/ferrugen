#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
use platform_core::{AppConfig, CoreError};
use std::collections::HashMap;

fn map(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

#[test]
fn missing_database_url_is_rejected() {
    let err = AppConfig::from_map(map(&[])).unwrap_err();
    assert!(matches!(err, CoreError::InvalidConfiguration(_)));
}

#[test]
fn invalid_port_is_rejected() {
    let m = map(&[
        ("DATABASE_URL", "postgres://localhost/db"),
        ("APP_PORT", "not-a-port"),
    ]);
    let err = AppConfig::from_map(m).unwrap_err();
    assert!(matches!(err, CoreError::InvalidConfiguration(_)));
}

#[test]
fn non_positive_session_ttl_is_rejected() {
    let m = map(&[
        ("DATABASE_URL", "postgres://localhost/db"),
        ("SESSION_TTL_SECONDS", "0"),
    ]);
    let err = AppConfig::from_map(m).unwrap_err();
    assert!(matches!(err, CoreError::InvalidConfiguration(_)));
}

#[test]
fn non_positive_login_window_is_rejected() {
    let m = map(&[
        ("DATABASE_URL", "postgres://localhost/db"),
        ("LOGIN_WINDOW_SECONDS", "0"),
    ]);
    let err = AppConfig::from_map(m).unwrap_err();
    assert!(matches!(err, CoreError::InvalidConfiguration(_)));
}

#[test]
fn non_positive_login_lockout_is_rejected() {
    let m = map(&[
        ("DATABASE_URL", "postgres://localhost/db"),
        ("LOGIN_LOCKOUT_SECONDS", "0"),
    ]);
    let err = AppConfig::from_map(m).unwrap_err();
    assert!(matches!(err, CoreError::InvalidConfiguration(_)));
}

#[test]
fn valid_defaults_are_applied() {
    let m = map(&[("DATABASE_URL", "postgres://localhost/db")]);
    let cfg = AppConfig::from_map(m).unwrap_or_else(|e| panic!("{}", e));
    assert_eq!(cfg.app_host, "127.0.0.1");
    assert_eq!(cfg.app_port, 8080);
    assert_eq!(cfg.session_ttl_seconds, 2592000);
    assert_eq!(cfg.login_max_attempts, 5);
    assert_eq!(cfg.login_window_seconds, 900);
    assert_eq!(cfg.login_lockout_seconds, 900);
}
