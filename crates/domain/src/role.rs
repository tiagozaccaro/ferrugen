#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Owner,
    Admin,
    Member,
    Viewer,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Owner => "owner",
            Role::Admin => "admin",
            Role::Member => "member",
            Role::Viewer => "viewer",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    TenantDelete,
    TenantTransfer,
    MemberInvite,
    MemberRemove,
    MemberRoleChange,
    SettingsManage,
    DataRead,
    DataWrite,
    BillingManage,
    AuditRead,
    JobManage,
}
