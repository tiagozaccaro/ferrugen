use crate::ApplicationError;
use domain::{Permission, Role};

pub fn authorize(role: Role, permission: Permission) -> Result<(), ApplicationError> {
    if role.allows(permission) {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}
