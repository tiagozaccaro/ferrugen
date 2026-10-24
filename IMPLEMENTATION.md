# Implementation agent protocol

1. Read `AGENTS.md`, `IMPLEMENTATION.md`, `review.md`, `TESTING.md`, `SECURITY.md`, and the relevant Spec Kit artifacts.
2. Confirm the exact acceptance criteria and package ownership before editing.
3. Propose a short plan if the task crosses more than two crates or changes a public contract.
4. Implement the smallest vertical slice that satisfies acceptance criteria.
5. Add or update tests; never disable a failing test to make CI green.
6. Run format, package check, package tests and relevant lint/license checks.
7. Invoke `@code-reviewer`, `@security-reviewer` for security-sensitive work, and `@test-reviewer` for behavior changes.
8. Fix confirmed findings and rerun affected checks.
9. Report changed files, decisions, commands run, test outcomes and unresolved risks.
10. Stop before push/merge/deploy unless explicitly authorized.

Do not claim checks passed unless their command output confirms it. Do not claim unimplemented packages are production-ready.
