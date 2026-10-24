use platform_core::{TenantId, UserId};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
}

impl Tenant {
    pub fn new(id: TenantId, name: String) -> Result<Self, DomainError> {
        let name = name.trim();
        if name.is_empty() || name.len() > 120 {
            return Err(DomainError::InvalidTenantName);
        }
        Ok(Self {
            id,
            name: name.to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Membership {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub role: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("tenant name must be between 1 and 120 characters")]
    InvalidTenantName,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_blank_tenant_name() {
        assert_eq!(
            Tenant::new(TenantId::new(), "  ".to_owned()),
            Err(DomainError::InvalidTenantName)
        );
    }
}
