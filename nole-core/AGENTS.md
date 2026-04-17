# nole-core

Core business logic crate for NoleAI. 16 source modules, ~2250 lines.

## WHERE TO LOOK

| Task | File | Key symbols |
|------|------|-------------|
| Add event type | `events.rs` | `Event` enum, `EventBus`, `EventPublisher` trait |
| Timer operations | `timer.rs` | `TimerService`, `TimerState`, `Timer` trait |
| Parse Obsidian vault | `vault.rs` | `VaultParser`, `Subject`, `DailyPlan`, `StudyTask` |
| Task scoring | `prioritization.rs` | `fsrs_priority_score()`, `prioritize_tasks()`, `EnergyProfile` |
| Anti-pattern alerts | `anti_patterns.rs` | `AntiPatternDetector`, `AntiPattern` enum, `SessionMetrics` |
| Anti-pattern wrapper | `anti_pattern.rs` | `DefaultAntiPatternDetector`, `MonitoredSessionTracker` |
| Knowledge graph data | `knowledge_graph.rs` | `KnowledgeGraph`, `KnowledgeNode`, `KnowledgeEdge` |
| Graph service | `knowledge_graph_service.rs` | `KnowledgeGraphService` |
| Emergency plans | `overload.rs` | `OverloadPlanGenerator` |
| Session tracking | `session_tracker.rs` | Session metrics recording |
| Socratic prompts | `socratic.rs` | Socratic questioning for study sessions |
| Plan templates | `templates.rs` | Template rendering for daily plans |
| Stress testing | `stress_test.rs` | `StressTestRunner` |
| File watching | `watcher.rs` | `VaultWatcher` with debounced events |
| Path security | `security.rs` | `validate_path_within()`, `create_secure_dir()` |

## CONVENTIONS

- Every public struct derives `Debug, Clone` + `Serialize, Deserialize` if it crosses crate boundary
- Error enums use `thiserror::Error` derive, result aliases match `{Module}Result<T>`
- Services take `Arc<dyn EventPublisher>` as dependency
- Regex parsing in `vault.rs` for Obsidian markdown format
- Anti-pattern thresholds are configurable via `HashMap<String, f32>` in constructor
- Knowledge graph edges use Spanish relation labels

## ANTI-PATTERNS

- `anti_pattern.rs` vs `anti_patterns.rs`: Two different files. `anti_patterns.rs` has the threshold-based `AntiPatternDetector` with Spanish messages. `anti_pattern.rs` has the `MonitoredSessionTracker` wrapper. Edit the right one.
- `SessionMetrics` here (`anti_patterns.rs`: start_time, duration_mins, breaks_taken, subject) differs from `the-crab-engram/src/storage.rs` (session_id, start_time, end_time, subject, breaks_taken, duration_without_breaks). Different types for different purposes.
- `vault.rs` has both `calculate_priority()` and `calculate_priority_by_mastery()` — identical logic, one takes `Subject`, the other takes `u8`. Prefer `calculate_priority_by_mastery`.
- Timer tick is driven externally (not self-scheduled). The HUD or caller must invoke `tick()` at the desired interval.
