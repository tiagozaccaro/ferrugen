# Pull request review policy

## Review order
1. Correctness and regressions
2. Security and tenant isolation
3. Data integrity, concurrency and migrations
4. Failure handling and observability
5. Test quality
6. Performance and maintainability
7. License and self-hosting implications

## Severity
- **P0 Blocker:** exploitable critical issue, cross-tenant data exposure, destructive data loss.
- **P1 High:** security boundary bypass, major broken flow, migration that can lose data.
- **P2 Medium:** meaningful bug, missing essential test, unsafe operational default.
- **P3 Low:** maintainability, minor UX, non-blocking improvement.

## Required output
For each finding include severity, file/line, scenario, impact, and concrete remediation. Do not report style preferences as bugs. Cite exact code paths. If no findings, state what was inspected and what was not verified.

## Reviewer constraints
Review agents are read-only. They must not edit code, commit, merge or deploy. They should not assume tests pass without running or seeing reliable CI results. No finding may be dismissed solely because a test does not cover it.
