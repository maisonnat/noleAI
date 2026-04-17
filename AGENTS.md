# PROJECT KNOWLEDGE BASE

**Generated:** 2026-04-17
**Commit:** ebb0c1e
**Branch:** master

## OVERVIEW

ADHD study assistant built in Rust. Cargo workspace with 4 crates + Tauri 2.x desktop HUD. Event-driven architecture with async channels. UI messages in Spanish.

## STRUCTURE

```
noleAI/
├── nole-core/           # Core logic: timer, vault, prioritization, anti-patterns, knowledge graph
├── nole-hud/            # Tauri 2.x desktop UI (separate nested workspace)
│   └── src-tauri/       # Rust backend for Tauri app
├── the-crab-engram/     # Storage backend: FSRS spaced repetition, JSON file persistence
├── notebooklm-rust-mcp/ # NotebookLM API: PDF extraction, quiz generation
├── vault/               # Obsidian vault: user config + generated daily plans
│   ├── Config/          # Materias.md (subjects + energy), preferences.md
│   └── HOY/             # Auto-generated daily study plans
├── specs/               # Implementation specs (plan/specification/tasks)
└── adhd/                # Research docs (ADHD study agent design)
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add new Event variant | `nole-core/src/events.rs` | Add to Event enum, then wire in publisher |
| Add Tauri command | `nole-hud/src/commands/*.rs` | Add function, register in `src-tauri/src/main.rs` invoke_handler |
| Modify timer logic | `nole-core/src/timer.rs` | TimerService uses Arc<Mutex<TimerState>> |
| Change prioritization | `nole-core/src/prioritization.rs` | FSRS scoring + deadline weighting |
| Add storage backend | `the-crab-engram/src/storage.rs` | Implement Storage trait (async_trait) |
| Parse vault config | `nole-core/src/vault.rs` | Regex-based parsing of Materias.md |
| PDF/quiz pipeline | `notebooklm-rust-mcp/src/lib.rs` | PDFProcessingPipeline orchestrates extract→quiz |
| Knowledge graph model | `nole-core/src/knowledge_graph.rs` | Nodes + edges with Mermaid rendering |
| Anti-pattern detection | `nole-core/src/anti_patterns.rs` | Thresholds configurable, Spanish alert messages |
| Overload mode | `nole-core/src/overload.rs` | Emergency simplified plans for low energy |
| Security (path validation) | `nole-core/src/security.rs` | validate_path_within, create_secure_dir |

## CONVENTIONS

- **Error types**: `thiserror::Error` derive → `{Name}Error` enum → `{Name}Result<T>` alias
- **Async traits**: `#[async_trait]` on ALL traits with async methods
- **Shared state**: `Arc<Mutex<T>>` for mutable, `Arc<dyn Trait>` for DI
- **Event flow**: Services publish `Event` via `EventPublisher` → HUD consumes via `async_channel`
- **Data models**: Always derive `Serialize, Deserialize, Debug, Clone`
- **Tests**: Inline `#[cfg(test)] mod tests` in each module
- **Spanish strings**: User-facing alerts/messages in Spanish (e.g., "Sesión larga detectada")
- **Workspace deps**: Shared versions in root `Cargo.toml [workspace.dependencies]`

## ANTI-PATTERNS (THIS PROJECT)

- **Duplicate `SessionMetrics`**: Defined in BOTH `nole-core/src/anti_patterns.rs` AND `the-crab-engram/src/storage.rs` with different fields. Check which one you need.
- **Duplicate anti-pattern files**: `anti_pattern.rs` (detector wrapper) vs `anti_patterns.rs` (threshold-based detection). Different purposes—know which you're editing.
- **nole-hud is nested workspace**: Has own `[workspace]` in Cargo.toml. `cargo test --workspace` from root does NOT include nole-hud. Build/test nole-hud separately.
- **No rustfmt/clippy config**: No formatting enforcement. Match existing style (4-space indent, Rust standard).

## COMMANDS

```bash
cargo build --release              # Build all workspace crates
cargo test --workspace             # Test nole-core, the-crab-engram, notebooklm-rust-mcp (NOT nole-hud)
cd nole-hud && cargo tauri dev     # Run HUD with hot reload
cd nole-hud && cargo tauri build   # Production build
```

## NOTES

- Data stored in `.engram-data/` (sessions.json, mastery.json, session_metrics.json)
- Vault config uses custom Markdown format with `## Subject | Nivel: N` and `Energy: HH:MM-HH:MM | Nivel: N`
- Timer default: 1500s (25 min). Long session threshold: 2700s (45 min)
- FSRS algorithm in `the-crab-engram/src/fsrs.rs` calculates next review intervals
- Knowledge graph edges use Spanish relations: "requiere", "relacionado con", "parte de"
- Tauri `AppState` in `main.rs` holds all shared state via `app.manage()`
