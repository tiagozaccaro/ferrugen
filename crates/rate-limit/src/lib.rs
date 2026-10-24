pub trait RateLimiter: Send + Sync {
    fn limit_by_user(&self, user_id: &str, endpoint: &str) -> Result<(), RateLimitError>;
    fn limit_by_tenant(&self, tenant_id: &str, endpoint: &str) -> Result<(), RateLimitError>;
    fn limit_by_ip(&self, ip: &str, endpoint: &str) -> Result<(), RateLimitError>;
}
#[derive(Debug)]
pub struct RateLimitError;
