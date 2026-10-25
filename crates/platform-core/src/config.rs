use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub database_url: String,
    pub app_host: String,
    pub app_port: u16,
    pub session_ttl_seconds: u64,
    pub login_max_attempts: u32,
    pub login_window_seconds: u64,
    pub login_lockout_seconds: u64,
}

impl AppConfig {
    pub fn new(
        database_url: String,
        app_host: String,
        app_port: u16,
        session_ttl_seconds: u64,
        login_max_attempts: u32,
        login_window_seconds: u64,
        login_lockout_seconds: u64,
    ) -> Result<Self, CoreError> {
        if database_url.trim().is_empty() {
            return Err(CoreError::InvalidConfiguration(
                "DATABASE_URL is required".to_string(),
            ));
        }
        if app_port == 0 {
            return Err(CoreError::InvalidConfiguration(
                "APP_PORT must be greater than 0".to_string(),
            ));
        }
        if session_ttl_seconds == 0 {
            return Err(CoreError::InvalidConfiguration(
                "SESSION_TTL_SECONDS must be greater than 0".to_string(),
            ));
        }
        if login_window_seconds == 0 {
            return Err(CoreError::InvalidConfiguration(
                "LOGIN_WINDOW_SECONDS must be greater than 0".to_string(),
            ));
        }
        if login_lockout_seconds == 0 {
            return Err(CoreError::InvalidConfiguration(
                "LOGIN_LOCKOUT_SECONDS must be greater than 0".to_string(),
            ));
        }
        Ok(Self {
            database_url,
            app_host,
            app_port,
            session_ttl_seconds,
            login_max_attempts,
            login_window_seconds,
            login_lockout_seconds,
        })
    }

    pub fn from_env() -> Result<Self, CoreError> {
        Self::from_map(std::env::vars())
    }

    pub fn from_map<I>(vars: I) -> Result<Self, CoreError>
    where
        I: IntoIterator<Item = (String, String)>,
    {
        let mut map = std::collections::HashMap::new();
        for (k, v) in vars {
            map.insert(k, v);
        }
        let database_url = map.get("DATABASE_URL").cloned().ok_or_else(|| {
            CoreError::InvalidConfiguration("DATABASE_URL is required".to_string())
        })?;
        let app_host = map
            .get("APP_HOST")
            .cloned()
            .unwrap_or_else(|| "127.0.0.1".to_string());
        let app_port = map
            .get("APP_PORT")
            .cloned()
            .unwrap_or_else(|| "8080".to_string())
            .parse::<u16>()
            .map_err(|_| {
                CoreError::InvalidConfiguration("APP_PORT must be a valid port".to_string())
            })?;
        let session_ttl_seconds = map
            .get("SESSION_TTL_SECONDS")
            .cloned()
            .unwrap_or_else(|| "2592000".to_string())
            .parse::<u64>()
            .map_err(|_| {
                CoreError::InvalidConfiguration(
                    "SESSION_TTL_SECONDS must be a positive integer".to_string(),
                )
            })?;
        let login_max_attempts = map
            .get("LOGIN_MAX_ATTEMPTS")
            .cloned()
            .unwrap_or_else(|| "5".to_string())
            .parse::<u32>()
            .map_err(|_| {
                CoreError::InvalidConfiguration(
                    "LOGIN_MAX_ATTEMPTS must be a positive integer".to_string(),
                )
            })?;
        let login_window_seconds = map
            .get("LOGIN_WINDOW_SECONDS")
            .cloned()
            .unwrap_or_else(|| "900".to_string())
            .parse::<u64>()
            .map_err(|_| {
                CoreError::InvalidConfiguration(
                    "LOGIN_WINDOW_SECONDS must be a positive integer".to_string(),
                )
            })?;
        let login_lockout_seconds = map
            .get("LOGIN_LOCKOUT_SECONDS")
            .cloned()
            .unwrap_or_else(|| "900".to_string())
            .parse::<u64>()
            .map_err(|_| {
                CoreError::InvalidConfiguration(
                    "LOGIN_LOCKOUT_SECONDS must be a positive integer".to_string(),
                )
            })?;
        Self::new(
            database_url,
            app_host,
            app_port,
            session_ttl_seconds,
            login_max_attempts,
            login_window_seconds,
            login_lockout_seconds,
        )
    }
}
