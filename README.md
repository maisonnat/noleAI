# NoleAI - ADHD Study Assistant

A Rust-based study assistant designed specifically for ADHD students, featuring real-time Pomodoro timers, intelligent task prioritization, spaced repetition, and knowledge graph visualization.

## Features

### Core MVP Features (P1)
- **Real Pomodoro Timer**: 25-minute focus sessions with pause/resume, break notifications, and anti-pattern detection for long sessions without breaks
- **Persistent Storage**: Session tracking, mastery level persistence, and session metrics via JSON-based file storage
- **Intelligent Prioritization**: ADHD-aware task scheduling using FSRS mastery scoring, deadline-aware weighting, and configurable energy profiles
- **Event Bus Integration**: Reactive HUD updates when Obsidian vault files change, with debounced file watching

### Advanced Features (P2)
- **PDF Processing**: Automatic text extraction from PDFs and quiz generation via NotebookLM integration with progress reporting
- **Knowledge Graph Visualization**: Interactive Mermaid-based graph with color-coded mastery levels (red=low, green=high), node click details, and relationship strength edge thickness
- **Overload Mode**: Emergency simplified plans for low-energy periods with breathing exercise UI, auto-exit timeout, and recovery plan generation
- **Anti-Pattern Detection**: Real-time alerts for unhealthy study patterns (e.g., 45+ min sessions without breaks) with dismissible banner UI

### Testing & Validation (P3)
- **Stress Testing**: Performance validation for PDF processing workflows with metrics reporting (average time, error tracking)

### UI Features
- **Dark/Light Theme**: Toggle between themes with CSS custom properties
- **Keyboard Accessibility**: ESC to dismiss modals, focus-visible outlines on all interactive elements
- **User Preferences**: Configurable via `vault/Config/preferences.md`

## Architecture

NoleAI is built as a Cargo workspace with the following components:

| Crate | Description |
|-------|-------------|
| `nole-core` | Core business logic: timer, events, vault parsing, prioritization, anti-patterns, stress testing, knowledge graph |
| `nole-hud` | Tauri 2.x desktop UI with commands for all features |
| `the-crab-engram` | Memory management, spaced repetition (FSRS algorithm), session/mastery persistence |
| `notebooklm-rust-mcp` | NotebookLM API integration for PDF text extraction and quiz generation |

### Event-Driven Architecture

```
VaultWatcher ──► EventBus ──► HUD (ObsidianVaultChanged)
TimerService ──► EventBus ──► HUD (TimerTick, TimerBreakRequested, AntiPatternDetected)
OverloadMode ──► EventBus ──► HUD (OverloadPlanGenerated)
```

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Tauri CLI: `cargo install tauri-cli`
- Node.js 16+ (for Tauri frontend development)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd noleAI
   ```

2. Build the workspace:
   ```bash
   cargo build --release
   ```

3. Set up the Obsidian vault:
   ```bash
   mkdir -p vault/Config vault/HOY
   ```

4. Create your subjects configuration at `vault/Config/Materias.md`:
   ```markdown
   # Materias

   ## Mathematics | Nivel: 2
   - Calculus
   - Linear Algebra
   - Probability

   ## Physics | Nivel: 3
   - Mechanics
   - Thermodynamics

   ## Energy Profiles
   Energy: 09:00-12:00 | Nivel: 4
   Energy: 14:00-17:00 | Nivel: 3
   Energy: 19:00-21:00 | Nivel: 2
   ```

5. (Optional) Configure preferences at `vault/Config/preferences.md`.

6. Run the HUD:
   ```bash
   cd nole-hud
   cargo tauri dev
   ```

## Usage

### 1. Configure Your Subjects
Edit `vault/Config/Materias.md` to add subjects, mastery levels (1-5), topics, and energy profiles.

### 2. Generate Daily Plan
Click "Generate Daily Plan" — the system uses FSRS scoring, deadline weighting, and your current energy level to produce an optimal task list limited to your energy capacity.

### 3. Pomodoro Timer
- Click "Start" for a 25-minute focus session
- "Pause"/"Resume" as needed
- A break modal appears when the session completes
- Anti-pattern alerts fire if you study 45+ minutes without a break

### 4. Track Progress
Sessions are saved to `.engram-data/` with mastery tracking via FSRS. Review overdue items through the mem_reviews API.

### 5. Knowledge Graph
Click "Show Knowledge Graph" to visualize subject relationships with mastery-coded colors and interactive node details.

### 6. Overload Mode
Click "Overload Mode" during low-energy periods for a simplified single-task plan with a guided breathing exercise. Auto-exits after 15 minutes.

### 7. Stress Testing
Click "Stress Test" and provide a PDF directory path to validate PDF processing performance with metrics reporting.

## Development

### Project Structure
```
noleAI/
├── nole-core/
│   └── src/
│       ├── events.rs              # EventBus, Event enum, EventPublisher trait
│       ├── timer.rs               # TimerService with anti-pattern emission
│       ├── vault.rs               # VaultParser, DailyPlan, energy profiles
│       ├── watcher.rs             # File watcher with debounce
│       ├── prioritization.rs      # FSRS scoring, deadline weighting, energy profiles
│       ├── session_tracker.rs     # Session metrics tracking
│       ├── anti_pattern.rs        # AntiPatternDetector, MonitoredSessionTracker
│       ├── overload.rs            # OverloadPlanGenerator, recovery plans
│       ├── knowledge_graph_service.rs  # Graph generation from subjects
│       └── stress_test.rs         # StressTestRunner
├── nole-hud/
│   ├── index.html                 # Full HUD with all UI features
│   └── src-tauri/src/main.rs     # Tauri app with all commands
├── the-crab-engram/
│   └── src/
│       ├── storage.rs             # Storage trait, FileStorage, SessionMetrics
│       ├── fsrs.rs                # FSRS algorithm
│       └── lib.rs                 # mem_save, mem_reviews, ensure_data_dir
├── notebooklm-rust-mcp/
│   └── src/
│       ├── pdf.rs                 # PDFProcessor trait, PDFTextExtractor
│       ├── client.rs              # NotebookLMClient, QuizRequest/Response
│       └── lib.rs                 # PDFProcessingPipeline, progress reporting
├── vault/
│   └── Config/
│       ├── Materias.md            # Subject and energy configuration
│       └── preferences.md         # User preferences
└── specs/                         # Implementation specifications
```

### Running Tests
```bash
cargo test --workspace
```

### Building for Production
```bash
cargo build --release
cd nole-hud
cargo tauri build
```

## Data Storage

| Path | Description |
|------|-------------|
| `.engram-data/sessions.json` | Session history |
| `.engram-data/mastery.json` | Subject mastery levels |
| `.engram-data/session_metrics.json` | Detailed session metrics |
| `vault/HOY/` | Generated daily plans |
| `vault/Config/` | User configuration |

## License

MIT License - See LICENSE file for details
