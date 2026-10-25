# Internal Ports Contract

**Feature**: `001-platform-foundation` | **Date**: 2026-10-25

These are the service-layer contracts for P001-P010. They are not HTTP endpoints. Concrete implementations live in `persistence`, `infrastructure`, `auth` and `audit`; tests may use in-memory fakes.

## Core types

```rust
pub struct UserId(pub Uuid);
pub struct TenantId(pub Uuid);
pub struct SessionId(pub Uuid);

pub struct Tenant {
    pub id: TenantId,
    pub name: String,
}

pub enum MembershipStatus {
    Invited,
    Active,
    Removed,
}

pub struct Membership {
    pub user_id: UserId,
    pub tenant_id: TenantId,
    pub role: Role,
    pub status: MembershipStatus,
}

pub struct TenantContext {
    user_id: UserId,
    tenant_id: TenantId,
    role: Role,
}
```

`TenantContext` fields remain private; construction goes through `resolve_tenant_context` or `from_verified_membership` inside the application crate.

## Role and permissions

```rust
pub enum Role {
    Owner,
    Admin,
    Member,
    Viewer,
}

pub enum Permission {
    TenantRead,
    TenantUpdate,
    TenantDelete,
    TenantTransfer,
    MemberRead,
    MemberAdd,
    MemberRemove,
    MemberRoleChange,
    SettingsManage,
    DataRead,
    DataWrite,
}

impl Role {
    pub fn allows(self, permission: Permission) -> bool;
}
```

Rules:
- Unknown/unsupported combinations return `false`.
- Only `Owner` may grant or remove `Owner`.
- `Admin` may manage non-owner roles and settings, but cannot delete/transfer the tenant or remove the last owner.
- The matrix is defined in `data-model.md` and compiled in code.

## Clock

```rust
pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}
```

Used for session expiry, throttle windows and deterministic tests.

## Password hashing

```rust
#[async_trait]
pub trait PasswordHasher: Send + Sync {
    async fn hash(&self, password: &str) -> Result<String, AuthError>;
    async fn verify(&self, hash: &str, password: &str) -> Result<bool, AuthError>;
}
```

The production implementation uses Argon2id. It must run CPU-heavy hashing off the async executor (for example, `tokio::task::spawn_blocking`) and must never log the password or hash.

## Session token codec

```rust
pub struct SessionToken(String);

impl SessionToken {
    pub fn expose(&self) -> &str;
}

impl fmt::Debug for SessionToken {
    // redacted; never prints the token
}

pub trait TokenCodec: Send + Sync {
    fn generate(&self) -> SessionToken;
    fn hash(&self, token: &SessionToken) -> [u8; 32];
}
```

The production implementation:
- Generates 32 bytes from a CSPRNG.
- Encodes the token for transport (hex or base64url).
- Hashes the encoded token with SHA-256 for storage.

## Auth entities

```rust
pub struct NewUser {
    pub id: UserId,
    pub email: String,          // normalized lowercase
    pub display_name: Option<String>,
    pub password_hash: String,  // Argon2id PHC string
}

pub struct User {
    pub id: UserId,
    pub email: String,
    pub display_name: Option<String>,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewSession {
    pub id: SessionId,
    pub user_id: UserId,
    pub token_hash: [u8; 32],
    pub expires_at: DateTime<Utc>,
}

pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub token_hash: [u8; 32],
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
```

`User` and `Session` must not derive `Debug` in a way that prints password hashes or token hashes.

## User repository

```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &NewUser) -> Result<(), AuthError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, AuthError>;
}
```

Duplicate email is reported as `AuthError::EmailTaken`; the database unique constraint is the final authority.

## Session repository

```rust
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &NewSession) -> Result<(), AuthError>;
    async fn find_by_token_hash(&self, hash: &[u8; 32]) -> Result<Option<Session>, AuthError>;
    async fn revoke(&self, id: SessionId, now: DateTime<Utc>) -> Result<(), AuthError>;
    async fn delete_expired(&self, now: DateTime<Utc>) -> Result<u64, AuthError>;
}
```

## Tenant and membership repositories

```rust
#[async_trait]
pub trait MembershipReader: Send + Sync {
    async fn membership_for(
        &self,
        user_id: UserId,
        tenant_id: TenantId,
    ) -> Result<Option<Membership>, ApplicationError>;
}

#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create_tenant_with_owner(
        &self,
        tenant: &Tenant,
        owner_id: UserId,
        now: DateTime<Utc>,
    ) -> Result<(), ApplicationError>;

    async fn find_by_id(&self, id: TenantId) -> Result<Option<Tenant>, ApplicationError>;
}

#[async_trait]
pub trait MembershipRepository: MembershipReader {
    async fn list_for_user(
        &self,
        user_id: UserId,
    ) -> Result<Vec<Membership>, ApplicationError>;

    async fn add_or_reactivate(
        &self,
        tenant_id: TenantId,
        user_id: UserId,
        role: Role,
        now: DateTime<Utc>,
    ) -> Result<(), ApplicationError>;

    async fn change_role(
        &self,
        tenant_id: TenantId,
        target_user_id: UserId,
        new_role: Role,
        now: DateTime<Utc>,
    ) -> Result<(), ApplicationError>;

    async fn remove_member(
        &self,
        tenant_id: TenantId,
        target_user_id: UserId,
        now: DateTime<Utc>,
    ) -> Result<(), ApplicationError>;
}
```

Rules:
- `create_tenant_with_owner` inserts the tenant and an active `owner` membership in one transaction.
- `change_role` and `remove_member` run in a transaction with `SELECT ... FOR UPDATE` on the tenant's memberships.
- Both operations return `ApplicationError::LastOwner` if they would leave zero active owners.
- `add_or_reactivate` updates the existing `(user_id, tenant_id)` row; it never inserts a duplicate.
- Every tenant-scoped read/write includes `tenant_id` in SQL.

## Login throttle

```rust
#[async_trait]
pub trait LoginThrottle: Send + Sync {
    async fn check_and_record(
        &self,
        normalized_email: &str,
        now: DateTime<Utc>,
    ) -> Result<(), AuthError>;

    async fn clear(&self, normalized_email: &str);
}
```

Production defaults: 5 failures per 15 minutes, 15-minute lockout, configurable. The in-memory implementation is per-process; multi-instance throttling is deferred.

## Audit (application-owned port)

```rust
pub enum AuditAction {
    LoginSucceeded,
    LoginFailed,
    Logout,
    RoleChanged,
    MemberRemoved,
}

pub enum AuditOutcome {
    Success,
    Denied,
    Failure,
}

pub struct AuditEvent {
    pub tenant_id: Option<TenantId>,
    pub actor_id: Option<UserId>,
    pub action: AuditAction,
    pub outcome: AuditOutcome,
    pub occurred_at: DateTime<Utc>,
}

#[async_trait]
pub trait AuditSink: Send + Sync {
    async fn record(&self, event: AuditEvent) -> Result<(), AuditError>;
}
```

`AuditAction`, `AuditEvent`, `AuditOutcome`, `AuditSink` and `AuditError` are defined in `crates/application/src/audit.rs`. `crates/audit` provides `TracingAuditSink` and `RecordingAuditSink` implementations; `crates/auth` emits login/logout events and `application::TenantService` emits role-change/member-removal events. Implementations must not log passwords, raw tokens, token hashes or other secrets. Persistence is P013.

**Dependency rule**: `application` must not depend on `auth` or `audit`. `auth` and `audit` may depend downward on `application` for the audit port; `infrastructure` wires the concrete sinks.

## Auth service

```rust
pub struct AuthService {
    // Arc<dyn ...> dependencies
}

impl AuthService {
    pub async fn register(&self, input: RegisterInput) -> Result<User, AuthError>;
    pub async fn login(&self, input: LoginInput) -> Result<SessionToken, AuthError>;
    pub async fn logout(&self, token: &SessionToken) -> Result<(), AuthError>;
    pub async fn authenticate(&self, token: &SessionToken) -> Result<AuthenticatedUser, AuthError>;
}
```

Behavior:
- `register`: normalize email, validate email and password length (12-128 characters), hash password, insert user; duplicate email -> `EmailTaken`; invalid length -> `WeakPassword`.
- `login`: normalize email, check throttle, find user, verify password; failure records a throttle attempt and emits `LoginFailed`; success clears throttle, creates a session, emits `LoginSucceeded`.
- `logout`: hash token, find session, set `revoked_at`, emit `Logout`; unknown/expired tokens are still treated as logged out (idempotent).
- `authenticate`: hash token, find session, reject missing/revoked/expired; return `AuthenticatedUser`.

## Tenant service

```rust
pub struct TenantService {
    // Arc<dyn ...> dependencies
}

impl TenantService {
    pub async fn create_tenant(
        &self,
        owner_id: UserId,
        name: &str,
    ) -> Result<Tenant, ApplicationError>;

    pub async fn resolve_context(
        &self,
        user_id: UserId,
        requested_tenant: TenantId,
    ) -> Result<TenantContext, ApplicationError>;

    pub async fn change_role(
        &self,
        actor: &TenantContext,
        target_user_id: UserId,
        new_role: Role,
    ) -> Result<(), ApplicationError>;

    pub async fn remove_member(
        &self,
        actor: &TenantContext,
        target_user_id: UserId,
    ) -> Result<(), ApplicationError>;
}
```

Behavior:
- `create_tenant` validates the tenant name and calls the atomic repository operation.
- `resolve_context` accepts the requested tenant ID only after an active membership lookup; missing/inactive -> `NotMember`.
- `change_role` authorizes `MemberRoleChange`; only an owner may grant/remove `owner`; last-owner guard is enforced by the repository transaction.
- `remove_member` authorizes `MemberRemove`; last-owner guard is enforced by the repository transaction.
- Role changes and removals emit audit events.

## Readiness port (API)

```rust
pub enum ReadinessState {
    Ready,
    DatabaseUnavailable,
    MigrationsFailed,
}

#[async_trait]
pub trait Readiness: Send + Sync {
    async fn check(&self) -> ReadinessState;
}
```

The production implementation checks a bounded `SELECT 1` and the recorded migration state. API tests use a fake readiness implementation to exercise 200/503 responses without a database.

## Errors

```rust
pub enum AuthError {
    InvalidCredentials,
    Disabled,
    Unavailable,
    EmailTaken,
    WeakPassword,
    TooManyAttempts,
    SessionExpired,
    SessionRevoked,
    Repository(String),
    Hashing(String),
}

pub enum ApplicationError {
    NotMember,
    Forbidden,
    LastOwner,
    InvalidInput(String),
    Repository(String),
}

pub enum AuditError {
    Sink(String),
}
```

Rules:
- No `unwrap`/`expect`/panic in production paths.
- Error messages returned to clients must not contain SQL, secrets or internal paths.
- Repository errors are mapped to safe service errors before reaching HTTP.
