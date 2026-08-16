# GET JOB READY — ROADMAP CALENDAR

> **August 14, 2026 → January 1, 2027**
> Become a credible junior/intern/contract-level **systems + developer tooling engineer: Rust-first, TypeScript-second, with a path to full-stack and Tauri desktop software**.

**The one rule:** every topic produces working evidence. Weekly deliverables, not daily schedules. You decide when/how you work. **Sunday is the competency deadline.**

---

## THE JANUARY TARGET

### Technical ability
- Strong Rust fundamentals, intermediate/advanced Rust, async Rust, Tokio, Axum
- CLI development, HTTP/networking, concurrency, filesystem/process interaction
- PostgreSQL + SQL, TypeScript, Node.js, React fundamentals
- Docker, Linux, Git/GitHub, CI/CD, testing, debugging, logging/tracing, basic production deployment

### Evidence
| Evidence | Target |
| --- | --- |
| Flagship project | **1** serious project (not ten) |
| OSS contributions | **3–8** meaningful, ideally merged PRs |
| GitHub profile | Polished |
| Resume | 1 page, one-pager |
| Portfolio | 1 site |
| Demo | 1 technical project demo/video |
| **The real test** | You can explain every significant technical decision **without hiding behind AI** |

### Competency, not coverage

The phases, dates, domains, and career direction in this roadmap stay fixed. What changes is the definition of progress: a topic is complete only after you independently implement it in a proportionate project and pass the Sunday evaluation.

```text
TOPIC
  -> understand the problem it solves
  -> use the relevant resource and code along
  -> close the resource and rebuild from memory
  -> experiment, break it intentionally, and fix it
  -> build the topic project in 01 - Skills.md
  -> explain the concept and implementation
  -> PASS / FAIL
```

Pass requires `Understanding = YES`, `Implementation = YES`, `Project = YES`, and `Explanation = YES`. A copied implementation, an unexplained implementation, or theory without working code is a fail. Record evidence in `02 - Weekly Tracker.md` and `03 - Project Archive.md`.

Topic projects are focused competency proofs. Existing weekly deliverables are composition projects that combine those proofs into progressively more realistic software:

```text
concept -> topic project -> composition project -> production system
        -> open source -> selected portfolio evidence -> job readiness
```

---

## MASTER CALENDAR

| # | Phase | Dates | Theme | Deliverable by Sunday |
| --- | --- | --- | --- | --- |
| P0 | SETUP | Aug 14 – Aug 16 | Establish the battlefield | **Environment completely ready** |
| 1 | RUST | Aug 17 – Aug 23 | Rust fundamentals (ownership, borrowing, Option, Result) | Topic proofs + 3 small Rust composition programs + Rustlings |
| 2 | RUST | Aug 24 – Aug 30 | Data modeling + Cargo | Topic proofs + first proper CLI: `filegrep` |
| 3 | RUST | Aug 31 – Sep 6 | Engineering (traits, generics, lifetimes, testing, Clippy) | Topic proofs + `filegrep` v2: maintainable software |
| 4 | SYSTEMS | Sep 7 – Sep 13 | Systems Rust (filesystem, processes, OS interaction) | `procpeek` / `devrun` |
| 5 | NETWORKING | Sep 14 – Sep 20 | Networking + HTTP (TCP, REST, JSON, serde, reqwest) | `devfetch`: polished API client CLI |
| 6 | ASYNC | Sep 21 – Sep 27 | Async Rust (futures, Tokio, tasks, channels) | `parallel-fetcher` |
| 7 | BACKEND | Sep 28 – Oct 4 | Axum (routing, handlers, extractors, middleware, state) | Rust REST API with real business logic |
| 8 | DATABASE | Oct 5 – Oct 11 | PostgreSQL + sqlx (schema, migrations, transactions) | Backend connected to PostgreSQL |
| 9 | TESTING | Oct 12 – Oct 18 | Testing, tracing, validation, "boringly reliable" | Hardened, trustworthy backend |
| 10 | LINUX+DOCKER | Oct 19 – Oct 25 | Linux operations + Docker (images, compose, volumes) | Containerized backend + Postgres: `docker compose up` |
| 11 | TYPESCRIPT | Oct 26 – Nov 1 | TypeScript + Node.js fundamentals | TS CLI talking to your Rust backend |
| 12 | REACT | Nov 2 – Nov 8 | React fundamentals (components, hooks, forms, API calls) | Frontend for your Rust backend |
| 13 | FLAGSHIP | Nov 9 – Nov 15 | **Flagship project begins — stack is frozen** | Spec, architecture, repo, issue tracker, roadmap |
| 14 | FLAGSHIP | Nov 16 – Nov 22 | Core engine (Rust, CLI, fs/process, config) | Core functionality works end-to-end |
| 15 | FLAGSHIP | Nov 23 – Nov 29 | Backend + data (API, Postgres, persistence, logging) | Usable product, not prototype |
| 16 | FLAGSHIP | Nov 30 – Dec 6 | Full stack (TypeScript interface / web UI) | Someone other than you can use it |
| 17 | FLAGSHIP | Dec 7 – Dec 13 | Production hardening (security, CI, Docker, tests) | Production-ish, showable to an engineer |
| 18 | OSS | Dec 14 – Dec 20 | Real open source contributions | **≥ 2 serious PRs submitted** |
| 19 | PORTFOLIO | Dec 21 – Dec 27 | GitHub, portfolio site, resume, interview prep | Applicant-ready artifacts |
| 20 | FINAL | Dec 28 – Jan 1 | Final war prep (no new tech!) | Everything finished + application spreadsheet |
| ✅ | **GO** | **Jan 1, 2027** | **Applying while continuing to improve** | Weekly cycle: Applications + OSS + Interview prep + Engineering |

---

## PHASE 0 — SETUP (Aug 14–16)

### Goal: establish the battlefield
- ✅ Rust toolchain (rustc 1.96.0) + Cargo + rustup
- ✅ rust-analyzer (Zed)
- ✅ Git 2.52.0 + GitHub (gh CLI, logged in as Weasert911)
- ✅ Node v24.11.0 + npm 11.6.1
- ⬜ TypeScript
- ⬜ PostgreSQL — **DEFERRED, install before Week 8** (winget install PostgreSQL.PostgreSQL.17)
- ✅ WSL2 + Ubuntu (for Linux work)
- ⬜ Docker — **SKIPPED FOR NOW, install before Week 10**
- ✅ Zed editor

### Deliverables
- [ ] GitHub repository for learning/work log (created: `job-ready-roadmap`)
- [ ] Skills document (`01 - Skills.md`)
- [ ] Weekly tracker (`02 - Weekly Tracker.md`)

---

## WEEK-BY-WEEK DETAIL

### WEEK 1 — Aug 17–23 · Rust fundamentals
**Learn:** variables, mutability, scalar/compound types, functions, expressions, control flow, ownership, borrowing, references, slices, structs, enums, `match`, `Option`, `Result`.

**Topic evidence:** complete the matching Rust projects in `01 - Skills.md` for variables, functions/control flow, ownership/borrowing, and structs/enums/result modeling. Use Rustlings as practice, not proof.

**Composition deliverable:** 3 small Rust programs (file analyzer, text statistics tool, terminal utility) that deliberately reuse the week's concepts.

**Sunday test — you can explain:**
1. Why does Rust have ownership?
2. What's the difference between `String` and `&str`?
3. What does borrowing mean?
4. Why does this code fail to compile?

---

### WEEK 2 — Aug 24–30 · Rust data modeling + CLI
**Learn:** structs, enums, pattern matching, methods, associated functions, modules, visibility, `Vec`, `HashMap`, `HashSet`, iterators, closures, Cargo, dependencies, crates, project structure.

**Topic evidence:** pass the matching projects for pattern matching/methods, modules/Cargo, and collections/iterators/closures.

**Composition deliverable — `filegrep`:**
```text
filegrep "TODO" ./project
filegrep "fn main" ./src
```
Parses arguments, traverses files, searches text, handles errors, produces useful output.

**Sunday standard:** someone else can clone it and run it.

---

### WEEK 3 — Aug 31 – Sep 6 · Rust engineering
**Learn:** error handling, custom errors, `Result`, `?`, traits, generics, lifetimes, smart pointers, testing, integration tests, documentation, formatting, Clippy.

**Topic evidence:** pass the projects for custom errors, traits/generics/lifetimes, smart pointers, and testing/tooling.

**Composition deliverable:** `filegrep` v2 — maintainable software: proper modules, tests, docs, error handling, CLI help, configuration, better architecture.

**Sunday standard:** you can explain your project's architecture.

---

### WEEK 4 — Sep 7–13 · Systems Rust begins
**Learn:** filesystem APIs, directories, paths, file handles, stdin/stdout/stderr, env vars, spawning processes, exit codes, signals, OS interaction.

**Understand:** `program → process → OS → filesystem`

**Topic evidence:** pass the matching Rust and Systems projects for filesystem, streams/environment, process execution, OS/process modeling, and permissions.

**Composition deliverable:** `procpeek` or `devrun` — launches commands, captures output, reports failures.

**Sunday standard:** you understand what actually happens when your program launches another program.

---

### WEEK 5 — Sep 14–20 · Networking + HTTP
**Learn:** IP, ports, TCP, HTTP, request/response, headers, status codes, JSON, REST, serialization. Rust: `serde`, `reqwest`.

**Topic evidence:** pass the networking, TCP, REST/HTTP, JSON serialization, and error-response projects.

**Composition deliverable — `devfetch`:** polished API-consuming CLI (requests, serialization, deserialization, errors, timeouts, bad responses).

---

### WEEK 6 — Sep 21–27 · Async Rust (major milestone)
**Learn:** futures, async/await, Tokio, tasks, spawning, channels, synchronization, concurrency vs parallelism, blocking vs non-blocking. Understand **why async exists**.

**Topic evidence:** pass the async/Tokio and concurrency projects. Measure or explain bounded versus unbounded work; merely using `.await` does not pass.

**Composition deliverable — `parallel-fetcher`:**
```text
parallel-fetcher urls.txt
```
Fetches 20 URLs concurrently, produces results.

**Sunday test:** What's a Future? What's a Tokio task? Why shouldn't blocking work happen inside async code? Concurrency vs parallelism?

---

### WEEK 7 — Sep 28 – Oct 4 · Rust backend
**Learn:** Axum: routing, handlers, extractors, middleware, JSON APIs, state, error responses, auth concepts.

**Topic evidence:** pass the Axum project plus the matching HTTP contract, validation, and typed error projects.

**Composition deliverable — REST API with real business logic:**
```text
POST /projects
GET /projects
GET /projects/:id
PATCH /projects/:id
DELETE /projects/:id
```
No Todo app unless radically expanded.

**Sunday standard:** you can create a working backend from scratch without a tutorial.

---

### WEEK 8 — Oct 5–11 · PostgreSQL
**Learn:** relational modeling, tables, PKs, FKs, constraints, joins, indexes, transactions, normalization, migrations, query performance basics. Rust: `sqlx`.

**Topic evidence:** pass all PostgreSQL projects and the `sqlx` persistence project. SQL design and query evidence must exist independently of framework code generation.

**Composition deliverable:** backend connected to PostgreSQL: schema, migrations, CRUD, transactions, validation, error handling.

**Sunday standard:** you can design a schema without an ORM generating everything.

---

### WEEK 9 — Oct 12–18 · Testing + production code
**Stop adding features.** Learn: unit/integration/API testing, test databases, mocking, error handling, validation, logging, `tracing`.

**Topic evidence:** pass the tracing, debugging, testing-strategy, health/graceful-failure, configuration/secrets, and relevant backend hardening projects.

**Composition deliverable:** backend is "boringly reliable": meaningful tests, structured logs, proper errors, input validation, edge cases.

**Sunday standard:** you trust your backend enough for another developer to use.

---

### WEEK 10 — Oct 19–25 · Linux + Docker
**Linux:** filesystem, permissions, processes, environment, SSH, package management, services, logs, networking basics, shell commands.
**Docker:** images, containers, Dockerfiles, volumes, networks, Compose.

> ⚠️ **Install Docker before this week.**

**Topic evidence:** pass the Linux and Docker topic projects. Use disposable drills where root access or destructive experiments are involved.

**Composition deliverable:** containerize backend + PostgreSQL. `docker compose up` = dev environment running.

**Sunday standard:** you can deploy/run software without an IDE holding your hand.

---

### WEEK 11 — Oct 26 – Nov 1 · TypeScript
**Learn:** types, interfaces, unions, intersections, generics, narrowing, utility types, modules, async/await, promises, error handling, package management, Node.js, HTTP clients.

**Topic evidence:** pass all non-React TypeScript projects. The Node CLI should reuse their modules rather than replace them with a tutorial implementation.

**Composition deliverable — TypeScript CLI talking to your Rust backend:**
```text
TypeScript CLI → HTTP → Rust backend → PostgreSQL
```

---

### WEEK 12 — Nov 2–8 · React + full stack
**Learn:** components, props, state, hooks, forms, API calls, routing, loading/error states, basic frontend architecture.

**Topic evidence:** pass both React projects with deliberate ownership of state and complete loading, empty, success, validation, and failure behavior.

**Composition deliverable:** frontend for your Rust backend:
```text
React → Rust/Axum → PostgreSQL
```

**Sunday standard:** you can independently connect frontend → backend → database.

---

### WEEK 13 — Nov 9–15 · FLAGSHIP BEGINS
**Stack is frozen. No new framework hunting.**

**Category (choose at this week):** scaffolding tool · dev environment manager · API development tool · local service manager · code/project analyzer · build/deployment tool · dev workflow automation · Rust/TS project manager.

**Deliverable:** written specification, architecture, repository, issue tracker, initial implementation, development roadmap.

**Selection gate:** choose a flagship because it solves a credible developer problem and composes proven competencies. Do not choose it to justify unlearned frameworks.

---

### WEEK 14 — Nov 16–22 · Flagship: core engine
**Prioritize:** architecture, Rust, CLI, filesystem/process interaction, configuration, error handling. Correct over beautiful.

**Deliverable:** core functionality works end-to-end.

---

### WEEK 15 — Nov 23–29 · Flagship: backend + data
**Add only what the product needs:** API, PostgreSQL, persistence, auth if necessary, background tasks, configuration, logging. No resume-driven features.

**Deliverable:** usable product, not a prototype.

---

### WEEK 16 — Nov 30 – Dec 6 · Flagship: full stack
**Add TypeScript interface:**
```text
Rust CLI → Rust backend → PostgreSQL
              ↑
       TypeScript web UI
```

**Deliverable:** someone other than you can actually use it.

---

### WEEK 17 — Dec 7–13 · Production hardening
**Learn/apply:** security basics, auth security, input validation, rate limiting concepts, secrets, env configuration, logging, graceful failures, DB backups, health checks.

**Add:** tests, CI, Docker, deployment.

**Deliverable:** production-ish — confidently showable to an engineer.

**Evidence gate:** deployment and hardening only pass when setup is reproducible, failures have been exercised, and operational decisions are documented. A green demo alone is insufficient.

---

### WEEK 18 — Dec 14–20 · Open source week
**Find real Rust/TypeScript repos**, beginner-friendly issues, docs, tests, bug fixes, small features, tooling.

**Contribution flow:**
```text
Issue → Understand codebase → Fork → Branch → Implement → Test → PR → Review → Revision → Merge
```

**Deliverable:** ≥ **2 serious PRs submitted** (no spam, no typo farming).

Complete the OSS competency projects in `01 - Skills.md`; maintainer feedback and revisions are evidence, not interruptions.

---

### WEEK 19 — Dec 21–27 · Portfolio + resume + interview prep
**GitHub:** pinned repos, README, docs, contribution history, profile.
**Portfolio:** About · Skills · Flagship project · OSS · Contact. Nothing more.
**Resume:** one page. What you built, what you contributed, technical depth, measurable results. No "passionate developer" fluff.

---

### WEEK 20 — Dec 28 – Jan 1 · Final war prep
**Do NOT start anything new. NO Kubernetes.**

**Finish:** flagship, documentation, portfolio, resume, GitHub, demo video, OSS PRs, LinkedIn/GitHub presence, application spreadsheet, target-company list.

**Practice explaining — Rust:** ownership, borrowing, lifetimes, traits, async, concurrency, error handling.
**Backend:** REST, HTTP, databases, transactions, authentication, caching concepts.
**Systems:** processes, threads, TCP, memory, filesystems, OS concepts.

**Your project (the biggest one):**
- Why did you build it? Why Rust? Why this architecture? Why this database?
- What was the hardest bug? What would you change?
- How does concurrency work? How would you scale it?
- What happens if the DB dies? How did you test it? What would you build next?

> If you can't explain your own project, **you're not ready.**

---

## JAN 1, 2027 — GO

> Not "I need to learn more before applying."
> **"I'm applying while continuing to improve."**

```text
Applications + OSS + Interview preparation + Engineering work + Continued Rust depth
```

---

## WEEKLY OPERATING SYSTEM

Work however you want (2 hours one day, 8 another, zero when school demands). **The weekly deliverable is what matters.**

| Metric | Weekly target |
| --- | --- |
| Coding | 15–25 focused hours |
| Rust | Every week |
| Shipped work | ≥ 1 meaningful deliverable |
| Documentation | ≥ 1 meaningful update |

> School comes first when necessary. No sacrificing academics for GitHub streaks.

### Weekly execution

1. Choose only the roadmap topics that realistically fit the week and create their rows/links in `03 - Project Archive.md`.
2. Learn and rebuild each topic independently before beginning its named project.
3. Keep topic projects proportional; do not add production ceremony that does not help prove the concept.
4. Reuse passed competencies in the week's composition deliverable.
5. On Sunday, evaluate every attempted topic separately in `02 - Weekly Tracker.md`.
6. Check the original skill only after all four gates pass. Failed topics stay visible and return to the next realistic work block.

Dates are planning constraints, not permission to fake mastery. If a topic fails, continue the roadmap where dependencies allow and schedule remediation; do not mark it complete to preserve the calendar.

---

## NON-NEGOTIABLE RULES

1. **Every topic produces a project** — watch/read -> understand -> close -> rebuild -> break -> fix -> build -> explain.
2. **AI allowed, but don't outsource your brain** — for every important piece of code, understand what it does, why it works, alternatives, failure modes, tradeoffs.
3. **Build before you feel ready** — "I don't know enough" is not a reason to wait.
4. **One flagship project** — `ONE GOOD PROJECT + REAL OSS CONTRIBUTIONS + SOLID FUNDAMENTALS`. That's the story.
5. **Small proof is not portfolio obligation** — archive every topic project, but polish only the few composition projects that best demonstrate engineering depth.
6. **Resources are disposable** — use the existing recommendation first, add another only for a specific gap, then stop watching and build.

---

## POST-JANUARY DESKTOP CONTINUATION

Tauri is the planned convergence point, not a shortcut around Rust, TypeScript, web fundamentals, or operating-system interaction. After the January target, build one desktop developer utility by reusing a proven Rust core and TypeScript/React interface.

The Tauri competency project should demonstrate commands and events, typed IPC boundaries, filesystem/process permissions, state, error propagation, packaging, and platform-aware behavior. Treat it as a portfolio project with tests for the Rust core, a reproducible build, screenshots/demo, architecture notes, and documented security decisions.

---

## FINAL PROGRESSION

```text
              YOU — JAN 2027
                    │
         ┌──────────┴──────────┐
         │                     │
       RUST                TYPESCRIPT
         │                     │
   ┌─────┼─────┐          ┌────┴────┐
   │     │     │          │         │
Systems Async DevTools   Node      React
   │     │     │          │         │
   └─────┴─────┘          └────┬────┘
             │                 │
             └───────┬─────────┘
                     │
                 PostgreSQL
                     │
                  Docker
                     │
                   Linux
                     │
               Git + CI/CD
                     │
                     ▼
             SHIPPED SOFTWARE
                     │
              ┌──────┴──────┐
              │             │
         FLAGSHIP APP     OSS PRs
              │             │
              └──────┬──────┘
                     ▼
               JOB APPLICATIONS
```

---

## DEFINITION OF READY

> **"Here's a serious piece of software I built. Here's its source. Here's the deployment. Here's how I tested it. Here's the architecture. Here's the OSS code I contributed. Here's what I know about Rust and systems. Give me a ticket and I'll figure it out."**

**That's the person a startup can consider hiring.**
