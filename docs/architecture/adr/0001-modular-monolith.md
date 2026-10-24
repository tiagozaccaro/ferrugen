# ADR 0001: Start as a modular monolith

- Status: Accepted
- Decision: Ship one API process, one worker process and one PostgreSQL database; keep capabilities isolated in Rust crates.
- Why: self-hosting simplicity, low operational overhead, transactional data integrity and easier local development.
- Consequences: crate boundaries and public interfaces must remain explicit; split services only when scale or isolation requirements justify the operational cost.
