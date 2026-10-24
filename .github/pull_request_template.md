# MANDATORY PR TEMPLATE — Every AI agent PR must use this format
## Summary
- 

## Issue
Closes #

## Acceptance criteria
- [ ] 

## Validation
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check` for affected packages
- [ ] `cargo test` for affected packages
- [ ] `cargo clippy` for affected packages
- [ ] Dependency/license impact reviewed

## Agent / Change Tracking
- Agent / harness used: _______
- Issue / spec link: _______
- Change scope (bounded): _______
- Review required (independent code / security / test): _______

## Independent reviews
- [ ] Code review
- [ ] Security review (when auth, tenancy, persistence, secrets or integrations change)
- [ ] Test review

## Risks / migration / operations
- 
