use application::authorize::authorize;
use domain::{Permission, Role};

#[test]
fn deny_by_default_denied() {
    assert!(authorize(Role::Viewer, Permission::DataWrite).is_err());
}

#[test]
fn owner_allowed_all() {
    assert!(authorize(Role::Owner, Permission::TenantDelete).is_ok());
}
