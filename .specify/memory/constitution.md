# Project Constitution

## I. Open-source and self-hosting first
Project code is MIT. Core features must work without paid SaaS accounts or proprietary deployment services. External services are optional adapters.

## II. Tenant isolation is mandatory
Users can belong to multiple tenants and tenants can have multiple users. Tenant membership and authorization are enforced on the server and tested against cross-tenant access.

## III. Reuse before duplication
Put stable cross-application behavior in reusable crates. Keep web, desktop, mobile, API and worker entry points thin. Avoid abstraction without a concrete second use case or testability benefit.

## IV. Modular monolith by default
Prefer clear crate boundaries and one deployable backend/worker topology before introducing distributed services.

## V. Secure defaults
Deny by default; validate input; protect secrets; audit sensitive actions; use explicit errors; test authorization failure paths.

## VI. Provider neutrality
Auth, AI, storage, email and job implementations must sit behind interfaces when replaceability is a real requirement. Optional providers cannot be mandatory for local core operation.

## VII. Testable changes
Every behavior change includes tests. Tenant-sensitive data access requires positive and negative tenant-isolation tests. CI must pass before merge.

## VIII. Dependency discipline
Prefer permissive licenses, audit direct/transitive dependencies, lock deployable apps, and record why new dependencies are needed.

## IX. Small, reviewable delivery
One issue per branch; small batches; PRs reference issues; independent review; squash merge only after CI and approval.

## X. Honest completion
Never label a scaffold as production-ready or a check as passed without evidence. Document target/toolchain gaps and unresolved risks.

### Governance
Amendments require a PR describing motivation, impact on existing modules, migration implications and updated tests/docs. Where an implementation conflicts with this constitution, stop and request a decision rather than silently weakening a principle.
