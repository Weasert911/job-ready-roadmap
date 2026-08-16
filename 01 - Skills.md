# SKILLS AND COMPETENCY CHECKLIST

> A checkbox means **independent, demonstrated competence**, not exposure. Evaluate every completed topic on Sunday and link its working project in `03 - Project Archive.md` before marking it complete.

## Completion gate

For every checklist item, follow this loop:

```text
Understand the problem -> use one useful resource -> code along -> close it
-> rebuild from memory -> experiment -> break it -> fix it -> build the project
-> explain it -> PASS / FAIL
```

Mark an item complete only when all four gates pass:

| Gate | Required evidence |
| --- | --- |
| Understanding | Explain the problem, mechanism, tradeoffs, and likely failure modes in your own words. |
| Implementation | Reproduce the core concept without copying the resource or accepting unexplained AI code. |
| Project | The named project runs, centers the target concept, and can be modified independently. |
| Explanation | Walk through the code and justify the important decisions without assistance. |

`YES + YES + YES + YES = PASS`. Any `NO = FAIL`: leave the item unchecked, record the gap in the Sunday review, and repeat the weakest part of the loop.

## Sizing

| Size | Typical effort | Expected scope |
| --- | ---: | --- |
| Tiny | 15-45 min | One concept, one executable or focused experiment |
| Small | 30-90 min | A useful utility with a few edge cases |
| Medium | 1-3 hr | Several interacting concepts, tests when useful |
| Large | 2-6+ hr | Concurrency, networking, UI, or multiple components |
| Major system | As needed | Cross-domain composition project with production practices |

The estimates are guides, not deadlines. Stop polishing once the project proves the competency. Later projects should reuse earlier skills.

---

## Rust

- [ ] **variables / mutability / scalar types / compound types** - `System Configuration Simulator` (Tiny): model immutable defaults, mutable runtime state, numeric/boolean/character values, tuples, and arrays.
- [ ] **functions / expressions / control flow** - `Exit-Code Decision Engine` (Tiny): convert mock command results into actions with functions, expression returns, `if`, and loops.
- [ ] **ownership / borrowing / references / slices** - `Zero-Copy Argument Inspector` (Small): parse and summarize borrowed string slices while showing ownership transfer and mutable/immutable borrowing.
- [ ] **structs / enums / match / Option / Result** - `Build Result Modeler` (Small): represent build jobs, optional metadata, success/failure states, and exhaustive transitions.
- [ ] **pattern matching / methods / associated functions** - `Command State Machine` (Small): parse command states and expose constructors and behavior through associated functions and methods.
- [ ] **modules / visibility / crates / Cargo** - `Workspace Metadata CLI` (Small): split parsing, domain, and output into modules with deliberate public boundaries and one external dependency.
- [ ] **Vec / HashMap / HashSet / iterators / closures** - `Log Frequency Analyzer` (Medium): aggregate levels and messages, deduplicate sources, and produce filtered rankings through iterator pipelines.
- [ ] **error handling / custom errors / ?** - `Config Validator` (Medium): load malformed and valid configuration, preserve error context, and return actionable custom errors.
- [ ] **traits / generics / lifetimes** - `Pluggable Diagnostic Formatter` (Medium): render borrowed diagnostics through generic code and multiple formatter implementations.
- [ ] **smart pointers** - `Shared Dependency Graph` (Medium): model owned nodes, shared references, interior mutability where justified, and cycle avoidance.
- [ ] **testing / integration tests / Clippy / formatting** - `Tested Argument Parser` (Medium): create unit and integration coverage, fixtures, documented behavior, clean formatting, and zero Clippy warnings.
- [ ] **filesystem APIs / paths / file handles** - `Directory Inspector` (Medium): traverse a tree, classify paths, read metadata, stream file content, and handle inaccessible entries.
- [ ] **stdin/stdout/stderr / environment variables** - `Environment Doctor` (Small): read configuration from env/stdin and separate machine-readable output from diagnostics.
- [ ] **spawning processes / exit codes / signals** - `Command Runner` (Medium): launch a child with arguments, capture streams, report exit status, and handle interruption/termination behavior.
- [ ] **networking (TCP/HTTP) / serde / reqwest** - `Dev Endpoint Probe` (Large): call endpoints with timeouts, serialize requests, deserialize responses, and distinguish protocol, status, and data errors.
- [ ] **async / futures / Tokio / tasks / channels / sync primitives** - `Concurrent Task Runner` (Large): bound concurrent work, communicate results over channels, coordinate shared state, cancel, and shut down cleanly.
- [ ] **Axum / routing / extractors / middleware / state / JSON APIs** - `Local Project Registry API` (Large): expose project metadata and scan operations with shared state, validation, middleware, JSON, and typed errors.
- [ ] **sqlx / migrations / transactions** - `Project Registry Persistence` (Major system): design migrations and transactional operations, then persist the Axum registry in PostgreSQL.
- [ ] **tracing / structured logging** - `Instrumented Job Pipeline` (Medium): add spans, fields, levels, request IDs, and configurable subscribers to a prior tool.
- [ ] **concurrency fundamentals** - `Parallel File Hasher` (Large): compare sequential, threaded, and bounded-worker execution and explain safety, contention, and measurement results.

## TypeScript

- [ ] **types / interfaces / unions / intersections** - `Typed Tool Manifest Parser` (Small): model manifests and result variants without unsafe assertions.
- [ ] **generics / narrowing / utility types** - `Typed Result Transformer` (Medium): transform generic API results using discriminated unions, guards, and utility types.
- [ ] **modules / async-await / promises / error handling** - `Async Command Queue` (Medium): organize a queue into modules, control promise failures, and report partial results.
- [ ] **package management (npm)** - `Publishable Config Package` (Small): create scripts, dependency boundaries, exports, semantic version notes, and a packed local install test.
- [ ] **Node.js / HTTP clients / API integration** - `Project Registry CLI` (Large): build a typed Node CLI that talks to the Rust API and handles timeouts, validation, and HTTP failures.
- [ ] **React: components / props / state / hooks** - `Local Service Monitor` (Large): display and control modeled services through composed components and deliberate state ownership.
- [ ] **React: forms / API calls / routing / loading + error states** - `Project Registry Console` (Major system): add validated forms, routes, API mutations, and explicit empty/loading/error/success states to the Rust system.

## Systems

- [ ] **program -> process -> OS -> filesystem model** - `Execution Lifecycle Lab` (Small): trace a program from invocation through process identity, environment, open files, and termination.
- [ ] **processes / threads / exit codes** - `Worker Topology Explorer` (Medium): run equivalent child-process and thread workloads and report status and isolation differences.
- [ ] **memory concepts** - `Memory Layout Explorer` (Medium): inspect stack/heap behavior, sizes, allocation, references, and ownership with documented observations.
- [ ] **TCP / networking fundamentals** - `TCP Echo Diagnostic` (Medium): implement a client/server pair with framing, disconnect handling, addresses, and port failures.
- [ ] **filesystem operations / permissions** - `Permission Auditor` (Medium): inspect a directory tree, report permission risks, and safely handle denied access and symlinks.

## DevTools

- [ ] **CLI development (argument parsing, help, exit codes)** - `Repo Doctor CLI` (Medium): provide subcommands, useful help, validated arguments, stable output, and meaningful exit codes.
- [ ] **logging / tracing** - `Traceable Build Wrapper` (Medium): wrap a build command with structured events, timings, correlation fields, and log-level control.
- [ ] **debugging** - `Fault Injection Lab` (Small): diagnose and document intentionally seeded parse, panic, and logic defects using a repeatable debugging process.
- [ ] **testing strategy (unit / integration / API / test DBs)** - `Registry Test Harness` (Large): define test boundaries and cover pure logic, process/HTTP integration, API behavior, and isolated database state.

## Backend

- [ ] **REST / HTTP / status codes / headers** - `HTTP Contract Sandbox` (Medium): implement resource operations with defensible methods, statuses, caching/content headers, and idempotency behavior.
- [ ] **JSON serialization** - `Versioned Manifest API` (Small): serialize and deserialize nested payloads with field naming, defaults, optional data, and invalid input tests.
- [ ] **authentication concepts** - `API Key Gateway` (Medium): authenticate hashed/revocable API keys and clearly document identity versus authorization.
- [ ] **error responses** - `Problem Details Layer` (Small): map domain failures into consistent, non-leaking JSON errors and HTTP statuses.
- [ ] **input validation** - `Project Spec Validator` (Small): validate structural and business rules and return field-level diagnostics.
- [ ] **rate limiting concepts** - `Request Budget Middleware` (Medium): implement a small per-client limiter and explain algorithm, fairness, storage, and distributed limitations.
- [ ] **health checks / graceful failures** - `Dependency Health Service` (Medium): expose liveness/readiness and drain in-flight work during shutdown or dependency failure.
- [ ] **secrets / environment configuration** - `Layered Configuration Loader` (Medium): merge defaults, files, environment, and CLI values without logging secrets.
- [ ] **basic production deployment** - `Deployed Registry Service` (Major system): deploy the existing backend with migrations, health checks, logs, rollback notes, and a runbook.

## Linux

- [ ] **filesystem / permissions** - `Least-Privilege Workspace Setup` (Small): create shared/private paths, inspect modes/ownership, and verify intended access.
- [ ] **processes / environment / services** - `User Service Unit` (Medium): run a tool as a managed service with environment configuration, restart behavior, and status inspection.
- [ ] **SSH / package management** - `Remote Bootstrap Script` (Medium): provision a clean Linux environment idempotently and verify remote access without embedding secrets.
- [ ] **logs / networking basics / shell commands** - `Linux Incident Drill` (Medium): diagnose a deliberately failed service using logs, process, socket, DNS, and shell inspection tools.

## Git

- [ ] **clone / commit / branch / merge / rebase** - `History Repair Lab` (Small): build and reconcile a disposable branch history, documenting when merge and rebase differ.
- [ ] **pull requests / code review** - `Reviewed Feature Change` (Medium): submit a focused change with tests, review your own diff, address comments, and preserve intent.
- [ ] **GitHub workflow / issues** - `Issue-to-Release Board` (Medium): plan a small tool through issues, labels, milestones, linked PRs, and a tagged release.

## Docker

- [ ] **images / containers / Dockerfiles** - `Containerized Rust Service` (Medium): create a reproducible multi-stage image, non-root runtime, and documented build/run commands.
- [ ] **volumes / networks / Compose** - `Service and Database Stack` (Large): connect backend and PostgreSQL with named volumes, health-aware startup, and private networking.
- [ ] **one-command dev environment (docker compose up)** - `Reproducible Dev Environment` (Large): make a clean checkout start, migrate, become healthy, and stop without manual setup.

## PostgreSQL

- [ ] **relational modeling / tables / PK / FK / constraints** - `Developer Workspace Schema` (Medium): model projects, commands, runs, and tags with database-enforced invariants.
- [ ] **joins / indexes / transactions** - `Run Analytics Queries` (Medium): implement multi-table reports, inspect query plans, add justified indexes, and protect a multi-step write.
- [ ] **normalization / migrations** - `Evolving Metadata Store` (Medium): normalize an initial design and migrate forward/backward without discarding representative data.
- [ ] **query performance basics** - `Slow Query Investigation` (Medium): seed realistic data, measure a poor query, inspect `EXPLAIN ANALYZE`, improve it, and record evidence.
- [ ] **schema design without ORM magic** - `Registry Schema Design Review` (Large): write SQL-first schema decisions, constraints, access patterns, and tradeoffs before integrating `sqlx`.

## OSS

- [ ] **understanding a foreign codebase** - `Codebase Orientation Report` (Medium): build the project, trace one behavior end-to-end, map modules/tests, and identify a viable change.
- [ ] **issue -> fork -> branch -> implement -> test -> PR -> review -> merge** - `Upstream Contribution` (Large): complete the full contribution workflow on a real repository and record review revisions.
- [ ] **>= 2 serious PRs submitted** - `Two Maintainer-Ready PRs` (Major evidence): submit scoped, tested changes that solve real project needs.
- [ ] **>= 3-8 meaningful contributions total (merged preferred)** - `Sustained OSS Track Record` (Major evidence): build a visible history across code, tests, tooling, or substantive documentation.

---

## Composition Gates

These are not replacements for topic projects. They prove that earlier competencies survive when combined.

| Gate | Combine | Required system | Class |
| --- | --- | --- | --- |
| Rust foundations | Variables through collections | Three focused Week 1 programs, then `filegrep` | Learning |
| Rust engineering | Errors, traits, testing, Cargo | Maintainable `filegrep` v2 | Learning |
| Systems tooling | Filesystem, processes, CLI, logging | `procpeek` or `devrun` | Portfolio candidate |
| Networked tooling | HTTP, serde, async, concurrency | `devfetch` then `parallel-fetcher` | Portfolio candidate |
| Backend | Axum, PostgreSQL, validation, tracing, tests | Project Registry service | Portfolio candidate |
| Full stack | Rust API, PostgreSQL, Node, React | Project Registry console | Portfolio candidate |
| Production | Linux, Docker, CI/CD, deployment | Deployed and operated system | Portfolio candidate |
| Desktop continuation | Rust core, TypeScript/React, IPC, OS integration | Tauri developer utility | Portfolio |
| Flagship | All relevant prior competencies | One serious developer tool | Portfolio |

## Evidence Tracker

| Evidence | Target | Status |
| --- | --- | --- |
| Topic projects | One passing project per checklist item | ⬜ |
| Composition projects | One passing project per reached gate | ⬜ |
| Flagship project | 1 serious | ⬜ |
| Tauri desktop utility | 1 complete, after web/full-stack foundation | ⬜ |
| OSS contributions | 3-8 (merged preferred) | ⬜ |
| GitHub profile | Polished | ⬜ |
| Resume | 1 page | ⬜ |
| Portfolio site | 1 | ⬜ |
| Demo video | 1 technical | ⬜ |
| Application spreadsheet | Target companies + pipeline | ⬜ |

## Resource Rule

1. Start with the roadmap's recommended resource when one exists.
2. Use a second resource only when the first leaves a specific gap.
3. Prefer official Rust, crate, TypeScript, platform, and framework documentation for exact behavior.
4. Stop consuming content as soon as you can attempt the memory rebuild.
5. Record the resource actually used in the Sunday review; the resource is context, not evidence.

## AI Gate

AI may explain, debug, review, suggest projects, and help with documentation. Before retaining AI-generated code, explain what it does, why it works, alternatives, assumptions, and failure modes. Rewrite the important core yourself when you cannot yet produce or defend it.
