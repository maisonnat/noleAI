# Tasks: NoleAI - Implementación Completa

**Input**: Design documents from `.specify/`
**Prerequisites**: PLAN.md (tech stack), SPECIFICATION.md (user stories)

**Tests**: Tests are OPTIONAL in this phase - focus on implementation first

**Organization**: Tasks grouped by feature to enable independent implementation and testing

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which feature this task belongs to
- Include exact file paths in descriptions

## Path Conventions

- **Rust Core**: `nole-core/src/`, `nole-core/Cargo.toml`
- **Tauri HUD**: `nole-hud/src-tauri/src/`, `nole-hud/index.html`
- **Engram**: `the-crab-engram/src/`, `the-crab-engram/Cargo.toml`
- **MCP**: `notebooklm-rust-mcp/src/`, `notebooklm-rust-mcp/Cargo.toml`
- **Vault**: `vault/` (Obsidian vault location)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Verify and complete initial project structure

- [ ] T001 Verify Cargo workspace structure in Cargo.toml
- [ ] T002 Verify all workspace member dependencies are correctly linked
- [ ] T003 [P] Verify Obsidian vault structure exists in vault/
- [ ] T004 [P] Verify basic Tauri project configuration in nole-hud/src-tauri/tauri.conf.json

**Checkpoint**: Project structure verified and ready for implementation

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before feature implementation

⚠️ **CRITICAL**: No feature work can begin until this phase is complete

- [ ] T005 Verify EventPublisher trait is properly defined in nole-core/src/events.rs
- [ ] T006 Verify EventBus implementation with broadcast channels in nole-core/src/events.rs
- [ ] T007 Verify all enum Event variants exist (SessionStarted, SessionEnded, ObsidianVaultChanged, OverloadModeActivated) in nole-core/src/events.rs
- [ ] T008 Configure tokio runtime integration for async events in workspace Cargo.toml
- [ ] T009 Verify vault module exists with all parsing functions in nole-core/src/vault.rs
- [ ] T010 Verify watcher module structure in nole-core/src/watcher.rs

**Checkpoint**: Foundation ready - feature implementation can now begin in parallel

---

## Phase 3: Feature 1 - Timer/Pomodoro Real (Priority: P1) 🎯 MVP

**Goal**: Implement a fully functional Pomodoro timer with real-time countdown, pause/resume functionality, and break notifications

**Independent Test**: Start a session, verify countdown updates in real-time, pause, resume, and receive break notification

### Implementation for Feature 1

- [ ] T011 [P] [F1] Create TimerState struct in nole-core/src/timer.rs with fields: duration, elapsed, is_paused, is_running
- [ ] T012 [P] [F1] Create Timer trait in nole-core/src/timer.rs with methods: start(), pause(), resume(), stop(), get_remaining_seconds()
- [ ] T013 [F1] Implement Tokio-based timer logic in nole-core/src/timer.rs using interval ticks
- [ ] T014 [F1] Add Timer events to Event enum in nole-core/src/events.rs: TimerTick { remaining_secs: u32 }, TimerBreakRequested
- [ ] T015 [F1] Implement TimerService in nole-core/src/timer.rs that combines TimerState and EventPublisher
- [ ] T016 [P] [F1] Add start_timer command in nole-hud/src-tauri/src/main.rs that initializes TimerService
- [ ] T017 [P] [F1] Add pause_timer command in nole-hud/src-tauri/src/main.rs
- [ ] T018 [P] [F1] Add resume_timer command in nole-hud/src-tauri/src/main.rs
- [ ] T019 [P] [F1] Add stop_timer command in nole-hud/src-tauri/src/main.rs
- [ ] T020 [F1] Create updateTimerDisplay JavaScript function in nole-hud/index.html to update countdown UI
- [ ] T021 [F1] Create handleTimerTick JavaScript function in nole-hud/index.html to process TimerTick events
- [ ] T022 [F1] Implement Tauri event listener in nole-hud/index.html for TimerTick events calling updateTimerDisplay
- [ ] T023 [F1] Add visual pause/play toggle buttons in nole-hud/index.html calling pause/resume commands
- [ ] T024 [F1] Implement break notification modal in nole-hud/index.html that triggers on TimerBreakRequested event
- [ ] T025 [F1] Add timer controls to AppState in nole-hud/src-tauri/src/main.rs including TimerService instance
- [ ] T026 [F1] Wire TimerService to EventBus in nole-hud/src-tauri/src/main.rs for TimerTick and TimerBreakRequested events

**Checkpoint**: At this point, Feature 1 (Timer) should be fully functional with real-time countdown and pause/resume

---

## Phase 4: Feature 2 - Persistence Real (Priority: P1)

**Goal**: Implement persistent storage for sessions, mastery levels, and review schedules using file-based storage

**Independent Test**: Save a session, restart application, verify session data persists correctly

### Implementation for Feature 2

- [ ] T027 [P] [F2] Create Storage trait in the-crab-engram/src/storage.rs with methods: save_session(), get_sessions(), get_mastery(), update_mastery()
- [ ] T028 [P] [F2] Create FileStorage struct in the-crab-engram/src/storage.rs implementing Storage trait
- [ ] T029 [F2] Implement JSON-based session storage in the-crab-engram/src/storage.rs with serialization/deserialization
- [ ] T030 [P] [F2] Create SessionRecord struct in the-crab-engram/src/models.rs with timestamp, subject, duration, breaks
- [ ] T031 [P] [F2] Create MasteryLevel struct in the-crab-engram/src/models.rs with subject, level, last_updated
- [ ] T032 [F2] Implement FSRS algorithm in the-crab-engram/src/fsrs.rs with calculate_next_review()
- [ ] T033 [F2] Refactor mem_save in the-crab-engram/src/lib.rs to use Storage trait instead of println
- [ ] T034 [F2] Implement get_mastery_level in the-crab-engram/src/lib.rs using Storage trait
- [ ] T035 [F2] Implement mem_reviews in the-crab-engram/src/lib.rs using FSRS algorithm and Storage trait
- [ ] T036 [F2] Add storage_path configuration to the-crab-engram/Cargo.toml via environment variable
- [ ] T037 [F2] Create .engram-data directory initialization in the-crab-engram/src/lib.rs if not exists
- [ ] T038 [F2] Add error handling for file I/O operations in the-crab-engram/src/storage.rs

**Checkpoint**: At this point, Feature 2 (Persistence) should save all data to files and restore on restart

---

## Phase 5: Feature 3 - PDF Processing Real (Priority: P2)

**Goal**: Implement actual PDF ingestion and quiz generation using NotebookLM MCP

**Independent Test**: Drop a PDF file, trigger processing, verify quiz questions are generated from actual PDF content

### Implementation for Feature 3

- [ ] T039 [P] [F3] Add pdf-extract crate dependency to notebooklm-rust-mcp/Cargo.toml
- [ ] T040 [P] [F3] Add serde_json crate dependency to notebooklm-rust-mcp/Cargo.toml for NotebookLM API integration
- [ ] T041 [F3] Create PDFProcessor trait in notebooklm-rust-mcp/src/pdf.rs with method extract_text()
- [ ] T042 [F3] Implement real PDF text extraction in notebooklm-rust-mcp/src/pdf.rs using pdf-extract
- [ ] T043 [F3] Create NotebookLMClient struct in notebooklm-rust-mcp/src/client.rs with API configuration
- [ ] T044 [F3] Implement create_notebook method in notebooklm-rust-mcp/src/client.rs for NotebookLM API
- [ ] T045 [F3] Implement add_source_pdf method in notebooklm-rust-mcp/src/client.rs to upload PDF
- [ ] T046 [F3] Implement generate_quiz method in notebooklm-rust-mcp/src/client.rs using NotebookLM artifact generation
- [ ] T047 [F3] Refactor generate_quiz_from_pdf in notebooklm-rust-mcp/src/lib.rs to use real PDF processing
- [ ] T048 [F3] Add error handling for PDF extraction failures in notebooklm-rust-mcp/src/pdf.rs
- [ ] T049 [F3] Add progress reporting for PDF processing in notebooklm-rust-mcp/src/lib.rs

**Checkpoint**: At this point, Feature 3 (PDF Processing) should ingest real PDFs and generate actual quizzes

---

## Phase 6: Feature 4 - Priorización Inteligente (Priority: P1)

**Goal**: Implement intelligent task prioritization based on ADHD energy levels, FSRS, and exam deadlines

**Independent Test**: Add multiple tasks with different priorities, verify daily plan shows optimal task order

### Implementation for Feature 4

- [ ] T050 [P] [F4] Create EnergyProfile struct in nole-core/src/prioritization.rs with time_slot, energy_level (1-5)
- [ ] T051 [P] [F4] Create TaskPriority struct in nole-core/src/prioritization.rs with subject, deadline, mastery_level, last_reviewed
- [ ] T052 [F4] Implement ADHD energy calculation in nole-core/src/prioritization.rs based on current time and energy profiles
- [ ] T053 [F4] Implement FSRS-based priority scoring in nole-core/src/prioritization.rs using mastery levels
- [ ] T054 [F4] Implement deadline-aware weighting in nole-core/src/prioritization.rs for urgent tasks
- [ ] T055 [F4] Create prioritize_tasks function in nole-core/src/prioritization.rs that returns sorted task list
- [ ] T056 [F4] Refactor generate_daily_plan in nole-core/src/vault.rs to use prioritize_tasks instead of "first subject"
- [ ] T057 [F4] Add energy profile configuration parsing from Config/Materias.md in nole-core/src/vault.rs
- [ ] T058 [F4] Implement fallback logic when no energy profile matches in nole-core/src/prioritization.rs
- [ ] T059 [F4] Add task count limit based on current energy level in nole-core/src/prioritization.rs

**Checkpoint**: At this point, Feature 4 (Prioritization) should generate intelligent daily plans

---

## Phase 7: Feature 5 - Knowledge Graph Visualization (Priority: P2)

**Goal**: Add interactive knowledge graph visualization to the HUD showing mastery levels and concept relationships

**Independent Test**: View knowledge graph, verify colors match mastery levels, clicking nodes shows details

### Implementation for Feature 5

- [ ] T060 [P] [F5] Add d3-graphviz or mermaid library dependency to nole-hud/index.html
- [ ] T061 [P] [F5] Create get_knowledge_graph Tauri command in nole-hud/src-tauri/src/main.rs
- [ ] T062 [F5] Implement get_knowledge_graph in nole-hud/src-tauri/src/main.rs calling nole-core knowledge_graph module
- [ ] T063 [F5] Create graph container HTML element in nole-hud/index.html
- [ ] T064 [F5] Create renderKnowledgeGraph JavaScript function in nole-hud/index.html using graph library
- [ ] T065 [F5] Add node color coding based on mastery level in renderKnowledgeGraph function
- [ ] T066 [F5] Implement interactive node click handlers in renderKnowledgeGraph function to show details
- [ ] T067 [F5] Add "Show Knowledge Graph" toggle button in nole-hud/index.html
- [ ] T068 [F5] Create KnowledgeGraphService in nole-core/src/knowledge_graph_service.rs to integrate with Engram
- [ ] T069 [F5] Implement sample knowledge graph data generation in KnowledgeGraphService from mock data
- [ ] T070 [F5] Add edge thickness visualization based on relationship strength in renderKnowledgeGraph function

**Checkpoint**: At this point, Feature 5 (Knowledge Graph) should display interactive visual graph in HUD

---

## Phase 8: Feature 6 - Integración EventBus-Watcher (Priority: P1)

**Goal**: Connect file watcher to event bus to enable reactive updates when vault changes

**Independent Test**: Modify a vault file, verify event is published and HUD updates accordingly

### Implementation for Feature 6

- [ ] T071 [P] [F6] Refactor start_watcher in nole-core/src/watcher.rs to accept EventPublisher parameter
- [ ] T072 [F6] Implement ObsidianVaultChanged event emission in watcher event handler in nole-core/src/watcher.rs
- [ ] T073 [F6] Add EventBus initialization to AppState in nole-hud/src-tauri/src/main.rs
- [ ] T074 [F6] Pass EventBus to start_watcher call in nole-hud/src-tauri/src/main.rs
- [ ] T075 [F6] Add ObsidianVaultChanged event listener in nole-hud/index.html to refresh task display
- [ ] T076 [F6] Implement auto-refresh of daily task when vault changes in nole-hud/index.html
- [ ] T077 [F6] Add debounce mechanism to avoid excessive refreshes in nole-core/src/watcher.rs

**Checkpoint**: At this point, Feature 6 (EventBus-Watcher) should automatically update HUD on vault changes

---

## Phase 9: Feature 7 - Modo Sobrecarga Real (Priority: P2)

**Goal**: Implement intelligent simplification logic for overload mode that generates simplified plans dynamically

**Independent Test**: Trigger overload mode, verify plan simplifies to single low-energy task, verify complexity reduces

### Implementation for Feature 7

- [ ] T078 [P] [F7] Create OverloadPlanGenerator struct in nole-core/src/overload.rs
- [ ] T079 [F7] Implement generate_emergency_plan function in nole-core/src/overload.rs with single-task output
- [ ] T080 [F7] Implement energy_aware_task_selection in nole-core/src/overload.rs based on current user energy
- [ ] T081 [F7] Add OverloadPlanGenerated event to Event enum in nole-core/src/events.rs
- [ ] T082 [F7] Refactor trigger_overload_mode in nole-hud/src-tauri/src/main.rs to generate emergency plan
- [ ] T083 [F7] Implement simplified plan display in nole-hud/index.html for overload mode
- [ ] T084 [F7] Add breathing exercise prompt in overload mode UI in nole-hud/index.html
- [ ] T085 [F7] Add recovery plan generation in nole-core/src/overload.rs for post-overload state
- [ ] T086 [F7] Implement timeout-based auto-exit from overload mode in nole-hud/src-tauri/src/main.rs

**Checkpoint**: At this point, Feature 7 (Modo Sobrecarga) should dynamically generate simplified emergency plans

---

## Phase 10: Feature 8 - Anti-Patterns Integration (Priority: P2)

**Goal**: Connect anti-pattern detection to timer and session flow for real-time alerts

**Independent Test**: Have a long session without breaks, verify alert triggers suggesting a break

### Implementation for Feature 8

- [ ] T087 [P] [F8] Create SessionTracker struct in nole-core/src/session_tracker.rs
- [ ] T088 [F8] Implement track_session_start in nole-core/src/session_tracker.rs to record session metrics
- [ ] T089 [F8] Implement track_session_end in nole-core/src/session_tracker.rs to finalize metrics
- [ ] T090 [F8] Wire AntiPatternDetector to SessionTracker in nole-core/src/session_tracker.rs
- [ ] T091 [F8] Add AntiPatternDetected event to Event enum in nole-core/src/events.rs
- [ ] T092 [F8] Emit AntiPatternDetected events from TimerService on long sessions in nole-core/src/timer.rs
- [ ] T093 [F8] Create anti-pattern alert UI component in nole-hud/index.html
- [ ] T094 [F8] Add AntiPatternDetected event listener in nole-hud/index.html to display alerts
- [ ] T095 [F8] Implement recommendations display from AntiPatternDetector in alert UI in nole-hud/index.html
- [ ] T096 [F8] Add session metrics persistence to Storage trait in the-crab-engram/src/storage.rs

**Checkpoint**: At this point, Feature 8 (Anti-Patterns) should detect and alert on problematic session patterns

---

## Phase 11: Feature 9 - Stress Testing Integration (Priority: P3)

**Goal**: Integrate stress testing module into main application flow for PDF processing validation

**Independent Test**: Run stress test with sample PDFs, verify report generation and metrics

### Implementation for Feature 9

- [ ] T097 [P] [F9] Create CLI command handler for stress tests in nole-core/src/stress_test.rs
- [ ] T098 [F9] Implement PDF ingestion in stress test using real notebooklm-rust-mcp in nole-core/src/stress_test.rs
- [ ] T099 [F9] Integrate real quiz generation in stress tests in nole-core/src/stress_test.rs
- [ ] T100 [F9] Add stress test command to Tauri in nole-hud/src-tauri/src/main.rs
- [ ] T101 [F9] Create stress test report viewer in nole-hud/index.html
- [ ] T102 [F9] Implement stress test result display with metrics in nole-hud/index.html
- [ ] T103 [F9] Add "Run Stress Test" button in nole-hud/settings UI in nole-hud/index.html

**Checkpoint**: At this point, Feature 9 (Stress Testing) should run integrated tests and display results

---

## Phase 12: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple features

- [ ] T104 [P] Add comprehensive error logging across all Tauri commands in nole-hud/src-tauri/src/main.rs
- [ ] T105 [P] Implement graceful error recovery for file I/O operations in vault module in nole-core/src/vault.rs
- [ ] T106 [P] Add user-friendly error messages in HUD UI for all failure scenarios in nole-hud/index.html
- [ ] T107 [P] Optimize timer tick frequency for performance in nole-core/src/timer.rs
- [ ] T108 [P] Add memory cleanup for long-running applications in nole-hud/src-tauri/src/main.rs
- [ ] T109 [P] Implement configuration file for user preferences in vault/Config/preferences.md
- [ ] T110 Add comprehensive README with setup instructions for all features
- [ ] T111 Add inline code documentation for public APIs in all modules
- [ ] T112 Performance optimization for large knowledge graphs in nole-core/src/knowledge_graph.rs
- [ ] T113 Security audit of file permissions and data storage paths
- [ ] T114 [P] Accessibility improvements for keyboard navigation in HUD in nole-hud/index.html
- [ ] T115 [P] Add theme configuration (light/dark mode support) in nole-hud/index.html

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all features
- **Features (Phase 3-11)**: All depend on Foundational phase completion
  - Features can then proceed in parallel (if team allows)
  - Or sequentially in priority order (F1, F4, F2, F6, F7, F8, F5, F3, F9)
- **Polish (Phase 12)**: Depends on all desired features being complete

### Feature Dependencies

- **Feature 1 (Timer) - P1**: Can start after Foundational - No dependencies on other features
- **Feature 2 (Persistence) - P1**: Can start after Foundational - No dependencies on other features
- **Feature 3 (PDF Processing) - P2**: Can start after Foundational - No dependencies
- **Feature 4 (Prioritization) - P1**: Can start after Foundational - Integrates with Feature 2 (mastery data)
- **Feature 5 (Knowledge Graph) - P2**: Can start after Foundational - Depends on Feature 2 (mastery levels)
- **Feature 6 (EventBus-Watcher) - P1**: Can start after Foundational - No dependencies
- **Feature 7 (Modo Sobrecarga) - P2**: Can start after Foundational - Integrates with Feature 4 (prioritization)
- **Feature 8 (Anti-Patterns) - P2**: Can start after Foundational - Integrates with Feature 1 (timer) and Feature 2 (persistence)
- **Feature 9 (Stress Testing) - P3**: Can start after Foundational - Integrates with Feature 3 (PDF processing)

### Within Each Feature

- Models/Structs before services/implementations
- Core implementation before UI integration
- Event definitions before event emission
- All tasks within feature complete before moving to next feature

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all P1 features (F1, F2, F4, F6) can start in parallel
- All models/structs within a feature marked [P] can run in parallel
- Different features can be worked on in parallel by different developers
- Polish tasks marked [P] can run in parallel after all features complete

---

## Parallel Example: Feature 1 (Timer)

```bash
# Launch all timer models/structs together:
Task: "Create TimerState struct in nole-core/src/timer.rs"
Task: "Create Timer trait in nole-core/src/timer.rs"

# Launch all Tauri commands together:
Task: "Add start_timer command in nole-hud/src-tauri/src/main.rs"
Task: "Add pause_timer command in nole-hud/src-tauri/src/main.rs"
Task: "Add resume_timer command in nole-hud/src-tauri/src/main.rs"
Task: "Add stop_timer command in nole-hud/src-tauri/src/main.rs"

# Launch all UI functions together:
Task: "Create updateTimerDisplay JavaScript function in nole-hud/index.html"
Task: "Create handleTimerTick JavaScript function in nole-hud/index.html"
```

---

## Implementation Strategy

### MVP First (Features F1, F2, F4, F6 - All P1)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all features)
3. Complete Phase 3: Feature 1 - Timer/Pomodoro Real
4. **STOP and VALIDATE**: Test Timer independently with countdown, pause, resume
5. Complete Phase 4: Feature 2 - Persistence Real
6. **STOP and VALIDATE**: Test session saving and retrieval
7. Complete Phase 6: Feature 6 - Integración EventBus-Watcher
8. **STOP and VALIDATE**: Test reactive updates from vault changes
9. Complete Phase 5: Feature 4 - Priorización Inteligente
10. **STOP and VALIDATE**: Test intelligent daily plan generation
11. **MVP DEPLOY**: Core NoleAI functionality complete

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add Feature 1 (Timer) → Test independently → Core HUD functionality
3. Add Feature 2 (Persistence) → Test independently → Data persistence
4. Add Feature 6 (EventBus-Watcher) → Test independently → Reactive updates
5. Add Feature 4 (Prioritization) → Test independently → Smart planning
6. Add Feature 7 (Modo Sobrecarga) → Test independently → Emergency support
7. Add Feature 8 (Anti-Patterns) → Test independently → Session health
8. Add Feature 5 (Knowledge Graph) → Test independently → Visual insights
9. Add Feature 3 (PDF Processing) → Test independently → Content ingestion
10. Add Feature 9 (Stress Testing) → Test independently → Performance validation
11. Each feature adds value without breaking previous features

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: Features F1 (Timer), F4 (Prioritization), F7 (Overload)
   - Developer B: Features F2 (Persistence), F5 (Knowledge Graph), F8 (Anti-Patterns)
   - Developer C: Features F3 (PDF Processing), F6 (EventBus-Watcher), F9 (Stress Testing)
3. Features complete and integrate independently
4. Polish phase: All developers work on parallel tasks

---

## Notes

- [P] tasks = different files, no dependencies
- [F#] label maps task to specific feature for traceability (F1-F9)
- P1 features (Timer, Persistence, Prioritization, EventBus) should be implemented first
- Each feature should be independently completable and testable
- Stop at any checkpoint to validate feature independently
- Feature dependencies: F5 and F7 depend on F2, F4; F8 depends on F1 and F2
- Focus on MVP features (P1) before advancing to P2/P3 features
- Avoid: vague tasks, same file conflicts, cross-feature dependencies that break independence
- Task count: 115 total tasks across 9 features + setup + foundational + polish
