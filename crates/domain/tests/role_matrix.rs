use domain::{Permission, Role};

#[test]
fn owner_allows_all() {
    let r = Role::Owner;
    assert!(r.allows(Permission::TenantDelete));
    assert!(r.allows(Permission::TenantTransfer));
    assert!(r.allows(Permission::AuditRead));
}

#[test]
fn admin_denied_delete_transfer_only() {
    let r = Role::Admin;
    assert!(!r.allows(Permission::TenantDelete));
    assert!(!r.allows(Permission::TenantTransfer));
    assert!(r.allows(Permission::DataRead));
    assert!(r.allows(Permission::MemberInvite));
}

#[test]
fn member_read_write_only() {
    let r = Role::Member;
    assert!(r.allows(Permission::DataRead));
    assert!(r.allows(Permission::DataWrite));
    assert!(!r.allows(Permission::MemberInvite));
    assert!(!r.allows(Permission::TenantDelete));
}
