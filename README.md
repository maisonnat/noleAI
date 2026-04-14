# NoleAI - ADHD Study Assistant

A Rust-based study assistant designed specifically for ADHD students, featuring real-time Pomodoro timers, intelligent task prioritization, spaced repetition, and knowledge graph visualization.

## Features

### Core MVP Features (P1)
- **Real Pomodoro Timer**: 25-minute focus sessions with pause/resume functionality
- **Persistent Storage**: Session tracking and mastery level persistence
- **Intelligent Prioritization**: ADHD-aware task scheduling based on energy levels and deadlines
- **Event Bus Integration**: Reactive updates when Obsidian vault changes

### Advanced Features (P2)
- **PDF Processing**: Automatic quiz generation from PDF content using NotebookLM
- **Knowledge Graph Visualization**: Interactive visual representation of concept relationships
- **Overload Mode**: Emergency simplified plans for low-energy periods
- **Anti-Pattern Detection**: Real-time alerts for unhealthy study patterns

### Testing & Validation (P3)
- **Stress Testing**: Performance validation for PDF processing workflows

## Architecture

NoleAI is built as a Cargo workspace with the following components:

- **nole-core**: Core business logic (timer, events, vault parsing, prioritization)
- **nole-hud**: Tauri-based desktop UI for the study assistant
- **the-crab-engram**: Memory management and spaced repetition (FSRS algorithm)
- **notebooklm-rust-mcp**: Integration with NotebookLM for PDF processing and quiz generation

## Getting Started

### Prerequisites
- Rust 1.70 or later
- Tauri CLI: `cargo install tauri-cli`
- Node.js (for Tauri development)

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
   cp vault/Config/Materias.md vault/Config/  # Edit as needed
   ```

4. Run the HUD:
   ```bash
   cd nole-hud
   cargo tauri dev
   ```

## Usage

### 1. Configure Your Subjects
Edit `vault/Config/Materias.md` to add your subjects and current mastery levels.

### 2. Generate Daily Plan
Click the "Generate Daily Plan" button in the HUD to create a personalized study schedule.

### 3. Use the Pomodoro Timer
- Click "Start" to begin a 25-minute focus session
- Use "Pause" and "Resume" as needed
- Take a 5-minute break when the timer completes

### 4. Track Progress
Sessions are automatically saved to `.engram-data/` with mastery level tracking using FSRS.

## Development

### Project Structure
```
noleAI/
├── nole-core/           # Core business logic
├── nole-hud/           # Tauri desktop UI
├── the-crab-engram/    # Memory & spaced repetition
├── notebooklm-rust-mcp/ # NotebookLM integration
├── vault/              # Obsidian vault (user data)
└── specs/              # Implementation specifications
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

### Building for Production
```bash
cargo build --release
cd nole-hud
cargo tauri build
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please read our contributing guidelines and code of conduct.

## Roadmap

- [ ] Complete MVP features (P1)
- [ ] Add PDF processing integration
- [ ] Implement knowledge graph visualization
- [ ] Add mobile companion app
- [ ] Multi-language support
- [ ] Cloud synchronization
