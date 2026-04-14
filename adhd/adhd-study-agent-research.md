# AI-Powered Study System for ADHD University Students
## Research & Implementation Blueprint

**Date**: 2026-04-15
**Scope**: How to build an AI-assisted study system for university students with ADHD, combining knowledge management (NotebookLM), persistent memory (agent memory systems), personal knowledge bases (Obsidian), and executive function support — orchestrated by a free LLM CLI agent.

---

## Table of Contents

1. [The Problem Space](#1-the-problem-space)
2. [ADHD & Executive Function: What the Science Says](#2-adhd--executive-function-what-the-science-says)
3. [Compensatory Strategies: Externalizing the Brain](#3-compensatory-strategies-externalizing-the-brain)
4. [Landscape of Existing Tools](#4-landscape-of-existing-tools)
5. [Why No Existing Tool Solves This](#5-why-no-existing-tool-solves-this)
6. [The Four Pillars Architecture](#6-the-four-pillars-architecture)
7. [Pillar 1: Knowledge Ingestion (NotebookLM)](#7-pillar-1-knowledge-ingestion)
8. [Pillar 2: Persistent Memory (Agent Memory Engines)](#8-pillar-2-persistent-memory)
9. [Pillar 3: Personal Knowledge Base (Obsidian)](#9-pillar-3-personal-knowledge-base)
10. [Pillar 4: Orchestration (LLM Agent)](#10-pillar-4-orchestration)
11. [Spaced Repetition Algorithms](#11-spaced-repetition-algorithms)
12. [Priority Algorithm for ADHD](#12-priority-algorithm-for-adhd)
13. [Anti-Pattern Detection](#13-anti-pattern-detection)
14. [UX Principles for ADHD](#14-ux-principles-for-adhd)
15. [Data Flow & Integration Patterns](#15-data-flow--integration-patterns)
16. [Implementation Blueprint](#16-implementation-blueprint)
17. [Appendix: Research Sources](#17-appendix-research-sources)

---

## 1. The Problem Space

### Who This Is For

A university student (any age) with ADHD who:
- Can learn material when presented correctly but struggles with **executive function** — the ability to plan, prioritize, start, and sustain effort
- Has access to course materials (PDFs, slides, textbooks) but finds it overwhelming to decide **what** to study, **when**, and **for how long**
- Forgets what they've already studied and repeats effort unnecessarily
- Gets paralyzed by the full scope of a semester and can't break it into manageable pieces
- Knows they should use spaced repetition (Anki, etc.) but can't maintain the habit

### What the System Should Do

Replace the executive functions that ADHD impairs:

| Impaired Function | What Breaks | What the System Does Instead |
|---|---|---|
| **Planning** | "I have 6 exams and don't know where to start" | Generates a daily plan with 2-3 prioritized tasks |
| **Prioritization** | Everything feels equally urgent | Calculates priority from deadlines, mastery, energy |
| **Task initiation** | Knowing what to do but being unable to start | "Start with THIS one thing right now" |
| **Time estimation** | "This will take 20 min" (takes 3 hours) | Tracks actual durations, suggests realistic blocks |
| **Working memory** | Forgetting what was just read | Quick recaps, "what do you remember?" prompts |
| **Sustained attention** | Brain shuts off after ~25 min | Pomodoro with check-ins at energy dips |
| **Emotional regulation** | Overwhelm → avoidance → guilt → more overwhelm | Graceful failure handling, micro-tasks, no guilt |

---

## 2. ADHD & Executive Function: What the Science Says

### The Barkley Model

Dr. Russell Barkley's research (the most cited in ADHD literature) identifies executive functions as **self-directed actions** that ADHD impairs:

1. **Working memory** (non-verbal) — Holding information in mind while using it
2. **Working memory** (verbal) — Internal self-talk, self-instruction
3. **Self-regulation of emotion** — Managing frustration, motivation
4. **Self-regulation of motivation** — Sustaining effort toward future rewards
5. **Planning and problem-solving** — Mental simulation of future actions
6. **Time perception** — Estimating duration, sensing time passing

**Key insight**: ADHD is not a deficit of attention. It's a deficit of **self-directed executive function**. The person can focus intensely on interesting things (hyperfocus) but cannot reliably direct their own attention, plan, or self-motivate for uninteresting-but-necessary tasks.

**Implication for AI**: The AI should serve as an **externalized executive function** — making decisions the person's brain can't reliably make, while letting the person do the actual learning (which they're capable of).

### Cognitive Load Theory

Sweller's Cognitive Load Theory (1988, updated through 2024) identifies three types of cognitive load:

- **Intrinsic load**: Complexity of the material itself (fixed)
- **Extraneous load**: Load from poor design, unnecessary decisions, context switching
- **Germane load**: Load from actual learning (desirable)

For students with ADHD, extraneous load is disproportionately high because:
- Deciding what to study requires executive function they lack
- Switching between materials costs more cognitive resources
- Maintaining an organizational system is itself a task that needs executive function

**The AI should minimize extraneous load to near zero**, freeing all cognitive resources for germane (learning) load.

### Spaced Repetition & the Forgetting Curve

Ebbinghaus (1885) established that memory decays exponentially without review. Spaced repetition schedules reviews at increasing intervals to maintain retention with minimal total study time.

Modern algorithms:
- **SM-2** (SuperMemo 2, 1987): The classic. Used by Anki. Simple, effective, but not personalized.
- **FSRS** (Free Spaced Repetition Scheduler, 2023): Machine-learning-based. 15-25% more efficient than SM-2. Personalizes to individual forgetting patterns. Now built into Anki 23.10+.
- **LECTOR** (2025): Uses LLMs to understand concept relationships, not just card-level repetition.

**ADHD-specific challenge**: Students with ADHD often abandon spaced repetition systems because:
- The daily review queue feels overwhelming ("I have 200 cards due")
- Maintaining cards requires consistent effort (executive function)
- The system doesn't adapt to energy fluctuations

**Design implication**: The system should auto-generate review items (from NotebookLM), calculate review schedules automatically (from agent memory), and present reviews in small batches (max 10 at a time).

---

## 3. Compensatory Strategies: Externalizing the Brain

Research on ADHD compensation (Frontiers in Psychology, 2025; MDPI, 2025) shows that successful adults with ADHD rely on **externalized systems**:

### What Works

| Strategy | Mechanism | AI Implementation |
|---|---|---|
| **External memory aids** | Offload working memory to physical/digital systems | Obsidian vault + agent memory |
| **Time externalization** | Use timers, alarms, visual schedules instead of internal time sense | Pomodoro with automated start/stop |
| **Decision reduction** | Pre-decide routine choices to avoid decision fatigue | AI generates daily plan, no "what to study?" moment |
| **Body doubling** | Having someone present (even virtually) increases focus | AI check-ins every 10-15 min during sessions |
| **Chunking** | Breaking large tasks into tiny pieces | "Read 2 pages" not "Study Chapter 5" |
| **Novelty injection** | Varying methods to maintain engagement | Quiz → flashcards → audio → debate mode |
| **Visual progress** | Seeing progress reduces overwhelm | Mastery dashboard, session logs |

### What Doesn't Work (for ADHD specifically)

| Strategy | Why It Fails |
|---|---|
| **Complex organizational systems** (Zettelkasten, PARA) | Requires executive function to maintain. Often abandoned after 2 weeks. |
| **Self-paced tools without structure** ("Study whatever you want") | Decision paralysis. The person can't choose. |
| **Guilt-based motivation** ("You missed 3 days, you're behind") | Triggers shame → avoidance cycle. |
| **One-size-fits-all scheduling** (everyone studies 2h/day) | Ignores energy fluctuations, hyperfocus days, crash days. |

### The "External Brain" Concept

Barkley describes ADHD compensation as building an **external prefrontal cortex** — a system that:
1. Holds your goals (so you don't forget them)
2. Breaks them into steps (so you don't get overwhelmed)
3. Tells you the next step (so you don't have to decide)
4. Tracks what's done (so you don't repeat or forget)
5. Adapts to your state (energy, mood, time available)

This is exactly what an AI agent can do — if it has persistent memory, knowledge access, and the right interface.

---

## 4. Landscape of Existing Tools

### Knowledge Ingestion Tools (AI → Study Material)

| Tool | What It Does | ADHD Gap |
|---|---|---|
| **Google NotebookLM** | Upload PDFs → chat, quizzes, flashcards, audio, mind maps | No prioritization, no session memory, no study planning. Just a Q&A tool. |
| **ChatGPT / Claude** | Upload files → ask questions | No persistence between sessions. No structured study flow. |
| **SciSpace** | Academic paper Q&A with citations | Research-focused, not study-focused. No spaced repetition. |
| **Perplexity** | Search + answer with sources | Not designed for studying. No memory of what you've learned. |

### Spaced Repetition Tools

| Tool | What It Does | ADHD Gap |
|---|---|---|
| **Anki** | SM-2/FSRS flashcards | Requires manual card creation. Review queue feels overwhelming. No integration with study planning. |
| **RemNote** | Notes + flashcards in one app | Good concept but complex. Still requires manual effort. |
| **Quizlet** | Flashcards + study modes | Commercial, limited free tier. No AI generation. |

### Productivity / Executive Function Tools

| Tool | What It Does | ADHD Gap |
|---|---|---|
| **Motion** (AI calendar) | Auto-schedules tasks based on priority | Expensive ($34/mo). Calendar-focused, not study-focused. Doesn't know your course material. |
| **Todoist** | Task management | Static lists. No AI prioritization. No integration with study material. |
| **Notion** | All-in-one workspace | Too flexible → decision paralysis. Complex setup. Not ADHD-optimized. |
| **Forest** | Focus timer (gamified) | Timer only. No study content integration. |
| **Focusmate** | Virtual body doubling | Human partner. No AI study assistance. |

### Knowledge Management (Second Brain)

| Tool | What It Does | ADHD Gap |
|---|---|---|
| **Obsidian** | Local-first markdown notes with graph | Powerful but requires discipline to maintain. No AI study features built in. |
| **Notion** | Database + notes + AI | Cloud-dependent. Can be overwhelming. |
| **Logseq** | Outliner + graph | Similar to Obsidian. Steeper learning curve. |
| **Roam Research** | Bidirectional linking | Expensive. Niche community. |

### AI Agent Frameworks

| Tool | What It Does | Relevance |
|---|---|---|
| **MCP (Model Context Protocol)** | Standard for connecting LLMs to tools | The integration protocol. Both NotebookLM MCP and agent memory systems use it. |
| **LangChain** | Framework for LLM applications | Overkill for this use case. Adds complexity. |
| **AutoGPT / BabyAGI** | Autonomous agents | Too autonomous. User loses control. Not suitable for ADHD (needs predictability). |

---

## 5. Why No Existing Tool Solves This

The gap is not in any single capability. It's in the **combination**:

```
NotebookLM:       Has knowledge ingestion     ❌ No memory, no planning, no prioritization
Anki:             Has spaced repetition        ❌ No knowledge ingestion, manual cards, no planning
Obsidian:         Has knowledge base           ❌ No AI study features, requires manual maintenance
Motion:           Has prioritization           ❌ No knowledge of your courses, expensive
Notion:           Has everything               ❌ Too complex, decision paralysis, not ADHD-optimized
```

**Nobody combines:**
1. ✅ AI-generated study material from course PDFs
2. ✅ Persistent memory of what you've studied and how well
3. ✅ Automatic prioritization based on deadlines + mastery + energy
4. ✅ Spaced repetition calculated automatically
5. ✅ A simple interface (not another app — use what you already have)

The system we're building fills this gap by orchestrating existing tools (NotebookLM, agent memory, Obsidian) rather than building everything from scratch.

---

## 6. The Four Pillars Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    STUDENT'S DAILY EXPERIENCE                │
│                                                              │
│   Opens Obsidian → sees HOY.md                              │
│   "Study DP for 25 min, then eigenvalues for 25 min"        │
│   Studies → takes quiz → result saved automatically         │
│   Closes Obsidian → done for the day                        │
│                                                              │
│   The student NEVER:                                         │
│   - Decides what to study (AI decides)                      │
│   - Creates flashcards (NotebookLM generates them)          │
│   - Tracks progress (agent memory tracks it)                │
│   - Plans the semester (AI plans day by day)                │
│                                                              │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┐
        ▼               ▼               ▼
   ┌─────────┐    ┌──────────┐    ┌──────────┐
   │Obsidian │    │ Gemini   │    │  Other   │
   │(vault)  │    │  CLI     │    │  tools   │
   └────┬────┘    └────┬─────┘    └──────────┘
        │              │
        │    ┌─────────┴──────────┐
        │    │                    │
        ▼    ▼                    ▼
   ┌──────────────┐    ┌──────────────────┐
   │  notebooklm  │    │  the-crab-engram │
   │  -rust-mcp   │    │       🦀         │
   └──────────────┘    └──────────────────┘
```

### Why Four Pillars, Not One Big Tool

Building one monolithic app would:
- Take 6+ months
- Reinvent things that already work (NotebookLM, Obsidian)
- Be fragile (if one part breaks, everything breaks)
- Not leverage the user's existing workflow

Orchestrating existing tools:
- Works in days, not months
- Each tool does what it's best at
- User already knows Obsidian
- Replacing any pillar doesn't break the system

---

## 7. Pillar 1: Knowledge Ingestion (NotebookLM)

### What NotebookLM Does Well

- **PDF → structured understanding**: Reads textbooks, slides, papers and creates a knowledge graph
- **Quiz generation**: Creates multiple-choice, short-answer, true/false questions
- **Flashcard generation**: Extracts key concepts into flashcard format
- **Audio overview**: Converts notes into podcast-style audio (good for auditory learners)
- **Mind maps**: Visual topic connections
- **Deep research**: Autonomous research with source discovery
- **Citation-grounded answers**: Every answer cites the source material

### The MCP Interface

An MCP server provides programmatic access to NotebookLM's features:

| MCP Tool | What It Does | Study Use Case |
|---|---|---|
| `notebook_create` | Create a notebook | One notebook per subject |
| `source_add_file` | Upload a PDF | Add textbook chapters, slides |
| `ask_question` | Q&A about sources | "Explain eigenvalue decomposition" |
| `artifact_generate(quiz)` | Generate quiz | "Test me on Chapter 5" |
| `artifact_generate(flashcards)` | Generate flashcards | Auto-generate review cards |
| `artifact_generate(audio)` | Generate audio | Listen during commute |
| `notebook_summary` | Get summary + suggested topics | "What are the main themes?" |
| `research_deep_dive` | Autonomous research | "Find more examples of DP in real-world" |

### Study Material Pipeline

```
Textbook PDF → source_add_file → NotebookLM indexes it
                                            ↓
                              Student asks: "Test me on Ch 5"
                                            ↓
                              artifact_generate(quiz, quantity=5)
                                            ↓
                              Quiz appears in Obsidian
                                            ↓
                              Student answers → results saved to agent memory
                                            ↓
                              Mastery updated → affects tomorrow's priority
```

---

## 8. Pillar 2: Persistent Memory (Agent Memory Engines)

### What Agent Memory Provides

A memory engine (like Crab Engram) gives the AI agent persistent knowledge about:

| Memory Type | What It Stores | How It Helps ADHD |
|---|---|---|
| **Observations** | Every study session, quiz result, topic studied | "You studied DP on Monday and scored 3/5" |
| **Beliefs** | Current understanding level per topic (Active → Confirmed → Contested) | "You believe you understand recursion, but your quiz scores say otherwise" |
| **Graph edges** | Relationships between topics (depends_on, supersedes) | "DP depends on recursion, which you're weak on" |
| **Knowledge capsules** | Synthesized summaries of what you know about a topic | Quick context injection before studying |
| **Knowledge boundaries** | Confidence levels per domain (novice → expert) | "You're novice in graphs, intermediate in sorting" |
| **Decay scores** | How recently and frequently you studied something | Drives spaced repetition scheduling |
| **Anti-patterns** | Recurring negative study patterns | "You always skip Mondays. Plan accordingly." |

### The 7 Auto-Learning Engines

An advanced memory system includes engines that observe and learn automatically:

1. **Consolidation Engine**: Finds duplicate memories, marks obsolete ones, detects conflicting information
2. **Anti-Pattern Detector**: Identifies recurring problems (hotspot topics, avoidance patterns)
3. **Smart Injector**: Provides relevant context to the LLM with token budget management
4. **Boundary Tracker**: Tracks what you know well vs. poorly per domain
5. **Capsule Builder**: Synthesizes knowledge summaries from individual observations
6. **Stream Engine**: Real-time event detection (studying pattern changes, new topics appearing)
7. **Graph Evolver**: Detects new relationships between topics over time

### Memory Operations for Study

| Operation | MCP Tool | Study Use Case |
|---|---|---|
| Save session | `mem_save` | "Studied DP for 25 min, scored 3/5 on quiz" |
| Search past sessions | `mem_search` | "When did I last study eigenvalues?" |
| Get reviews for today | `mem_reviews` | "What should I review based on spaced repetition?" |
| Get mastery stats | `mem_stats` | "How well do I know each subject?" |
| Detect anti-patterns | `mem_antipatterns` | "What patterns of procrastination do I have?" |
| Relate topics | `mem_relate` | "DP depends on recursion" |
| Get graph | `mem_graph` | Visual map of knowledge connections |
| Get beliefs | `mem_beliefs` | "What do I think I know? Is it confirmed?" |

---

## 9. Pillar 3: Personal Knowledge Base (Obsidian)

### Why Obsidian (Not Notion, Not a Custom App)

| Reason | Detail |
|---|---|
| **Local-first** | Files on your disk. No cloud dependency. No subscription. |
| **Markdown** | Universal format. Any tool can read/write it. |
| **Graph view** | Visual connections between topics (matches Crab Engram's graph) |
| **Plugins** | 1000+ plugins. Terminal, API, calendar, tasks, etc. |
| **Already known** | If the student already uses Obsidian, zero learning curve |
| **AI-friendly** | Markdown is the native format of LLMs. Easy to read, write, and parse. |
| **Community** | Huge academic community. Templates for every workflow. |

### Vault Structure for ADHD Study System

```
Universidad/
│
├── GEMINI.md                         # Agent's persistent context/personality
│
├── 📋 HOY.md                         # Auto-generated daily plan
│   ---
│   date: 2026-04-15
│   generated_by: gemini-cli
│   ---
│   ## Plan de hoy
│   1. [[Algoritmos/DP|Algoritmos — DP]] (25 min)
│      - Objetivo: Entender memoization vs tabulation
│      - Acción: Hacer quiz de 5 preguntas
│   2. [[Álgebra/Eigenvalues|Álgebra — Eigenvalues]] (25 min)
│      - Objetivo: Definición + propiedades
│   3. [[Historia/Cap5|Historia — Cap 5]] (25 min)
│      - Objetivo: Lectura ligera, sin presión
│
├── 📊 Progreso.md                    # Auto-updated mastery dashboard
│   | Materia | Mastery | Última sesión | Examen |
│   |---------|---------|---------------|--------|
│   | Algoritmos | 45% | Hoy | 5 días |
│   | Álgebra | 35% | Ayer | 12 días |
│   | Historia | 65% | Hace 3 días | 20 días |
│
├── 🧠 Repaso.md                      # Spaced repetition for today
│   ## Para repasar hoy
│   - [ ] [[Algoritmos/DP#Memoization|DP: Memoization]] — confianza baja
│   - [ ] [[Álgebra/Eigenvalues#Definición|Eigenvalues definición]] — confianza media
│
├── Config/
│   ├── Materias.md                   # Subject definitions (deadlines, weights)
│   └── Mi energía.md                 # ADHD energy profile
│
├── Algoritmos/
│   ├── Notas/                        # Class notes
│   ├── Quiz Results/                 # Auto-generated quiz logs
│   ├── DP Patterns.md                # Synced from NotebookLM
│   └── Exam Prep.md                  # Exam checklist
│
├── Álgebra Lineal/
│   └── ...
│
├── Sesiones/                         # Study session logs
│   ├── 2026-04-15-0900-algoritmos.md
│   └── daily-summaries/
│       └── 2026-04-15-resumen.md
│
└── .obsidian/                        # Obsidian config
    └── plugins/
        └── local-rest-api/           # For MCP integration
```

### Key Obsidian Plugins for This System

| Plugin | Purpose |
|---|---|
| **Local REST API** | Lets MCP tools read/write notes via HTTP |
| **Templater** | Template system for HOY.md, session logs |
| **Dataview** | Query-based dashboards (alternative to auto-generated Progreso.md) |
| **Tasks** | Checkbox management with dates |
| **Calendar** | Visual session history |
| **Terminal** | Run `gemini` commands from within Obsidian |

---

## 10. Pillar 4: Orchestration (LLM Agent)

### Why Gemini CLI

| Factor | Gemini CLI | Claude Code | Cursor |
|---|---|---|---|
| **Price** | Free (1000 req/day) | $20/month | $20/month |
| **Context window** | 1M tokens | 200K | 128K |
| **MCP support** | ✅ | ✅ | ✅ |
| **Scriptable** | ✅ headless mode | ✅ | ❌ |
| **Google ecosystem** | Native (NotebookLM is Google) | No | No |
| **Custom commands** | ✅ GEMINI.md | ✅ CLAUDE.md | ✅ |
| **Terminal-first** | ✅ | ✅ | ❌ (IDE) |

For a student: **free wins**. 1000 requests/day is more than enough for study orchestration.

### The Agent's Role

The LLM agent is NOT the study tool. It's the **orchestrator** — the "executive function" layer:

```
Student says: "¿Qué estudio?"

Agent:
  1. Reads Config/Materias.md → knows subjects, deadlines, weights
  2. Calls crab-engram: mem_reviews → knows what needs review
  3. Calls crab-engram: mem_stats → knows current mastery levels
  4. Reads Config/Mi energía.md → knows peak hours
  5. Runs priority algorithm (deterministic, not LLM)
  6. Writes HOY.md in Obsidian
  7. Says: "Algoritmos DP, 25 min. Empezar ahora."
```

The agent decides. The student executes. Minimal cognitive load.

### GEMINI.md: The Agent's Memory

GEMINI.md is a file in the vault root that Gemini CLI reads as persistent context. It's the "system prompt" for the study agent:

```markdown
# Study Coach for ADHD

## Identity
I am a study coach. My job is to reduce cognitive load for a university
student with ADHD. I decide what to study, when, and how. The student
executes. I never ask "what would you like to study?" — I tell them.

## Tools I Use
- notebooklm: Generate quizzes, flashcards, summaries from course PDFs
- crab-engram: Persistent memory of sessions, mastery, spaced repetition
- file system: Read/write Obsidian vault notes

## Rules
1. Maximum 3 tasks per day. Never more.
2. I decide. Student does. No choices to make.
3. Hard subjects at peak energy hours only.
4. Missed yesterday? Recalculate silently. No guilt.
5. 25-minute sessions. 5-minute breaks. No exceptions.
6. Overwhelm → reduce to 1 micro-task or suggest break.
```

### Custom Commands

Gemini CLI supports custom commands defined in settings:

```json
{
  "customCommands": {
    "hoy": {
      "prompt": "Read Config/Materias.md, query crab-engram for reviews and stats. Generate HOY.md with max 3 prioritized sessions. Write it to the vault."
    },
    "test": {
      "prompt": "Ask the student which topic. Query notebooklm to generate a 5-question quiz. Present questions one by one. Score answers. Save results to crab-engram."
    },
    "repaso": {
      "prompt": "Query crab-engram mem_reviews. Present today's review items as Obsidian checkboxes with links."
    },
    "no-puedo": {
      "prompt": "Mode: overwhelm. Offer exactly ONE option: either a 10-minute micro-task or a 15-minute break. No other choices."
    }
  }
}
```

Usage: `gemini hoy`, `gemini test`, `gemini repaso`, `gemini no-puedo`

---

## 11. Spaced Repetition Algorithms

### SM-2 (SuperMemo 2, 1987)

The classic algorithm used by Anki:

```
For each flashcard:
  After review, user rates quality 0-5
  IF quality >= 3:
    interval = interval * easiness_factor
  ELSE:
    interval = 1 (reset)
  easiness_factor = max(1.3, EF + (0.1 - (5-q) * (0.08 + (5-q) * 0.02)))
```

**Pros**: Simple, well-understood, widely implemented
**Cons**: Not personalized. Doesn't learn individual forgetting patterns.

### FSRS (Free Spaced Repetition Scheduler, 2023)

Machine-learning-based replacement for SM-2:

```
Uses 3 parameters per card:
  - difficulty (D): How hard the card is for this user
  - stability (S): How long the memory lasts
  - retrievability (R): Current probability of recall

After each review:
  Update D, S based on user's actual performance
  Calculate optimal next review date where R ≈ 0.9 (90% recall probability)
```

**Pros**: 15-25% more efficient than SM-2. Personalizes to individual forgetting. Built into Anki 23.10+.
**Cons**: More complex. Requires sufficient review data to calibrate.

### Implementation for This System

The agent memory system already has `mem_reviews` which calculates what needs review. The algorithm can be:

```python
def calculate_review_items(subjects, now):
    """Determine what needs review today"""
    items = []
    for subject in subjects:
        for topic in subject.topics:
            days_since = (now - topic.last_reviewed).days
            confidence = topic.confidence  # 0-100 from quiz scores
            
            # Simple FSRS-inspired calculation
            if confidence > 80:
                interval = days_since * 2.5  # Confident → longer intervals
            elif confidence > 50:
                interval = max(1, days_since * 1.5)  # Medium
            else:
                interval = 1  # Not confident → review tomorrow
            
            if days_since >= interval:
                items.append(ReviewItem(
                    topic=topic,
                    urgency=100 - confidence,
                    estimated_time=5  # minutes
                ))
    
    # Sort by urgency, limit to 10 (don't overwhelm)
    return sorted(items, key=lambda x: x.urgency, reverse=True)[:10]
```

---

## 12. Priority Algorithm for ADHD

### Inputs

```python
inputs = {
    # From subject definitions
    "deadline_days": int,       # Days until exam
    "exam_weight": float,       # % of final grade (0-100)
    "difficulty": int,          # Self-rated 1-10
    
    # From agent memory
    "current_mastery": float,   # 0-100 from quiz scores
    "days_since_studied": int,  # Days since last session
    "confusion_topics": int,    # Number of unresolved confusions
    
    # From energy profile
    "is_peak_hour": bool,       # Is it the student's peak hour?
    "is_hard_subject": bool,    # difficulty >= 7
    
    # From session history
    "sessions_this_week": int,  # How many sessions this subject this week
    "last_quiz_score": float,   # Most recent quiz performance
}
```

### Scoring Formula

```python
def priority_score(inputs, now_hour, energy_profile):
    score = 0
    
    # 1. Deadline urgency (30% weight) — exponential as deadline nears
    if inputs["deadline_days"] <= 0:
        score += 30  # EXAM IS TODAY
    elif inputs["deadline_days"] <= 3:
        score += 27
    elif inputs["deadline_days"] <= 7:
        score += 21
    elif inputs["deadline_days"] <= 14:
        score += 12
    else:
        score += 6
    
    # 2. Mastery gap (20% weight) — lower mastery = higher priority
    mastery_gap = max(0, 80 - inputs["current_mastery"])
    score += mastery_gap * 0.2
    
    # 3. Confusion level (15% weight)
    score += min(inputs["confusion_topics"] * 7.5, 15)
    
    # 4. Recency penalty (15% weight) — haven't studied in a while
    recency = min(inputs["days_since_studied"] * 5, 15)
    score += recency
    
    # 5. Exam weight (10% weight)
    score += inputs["exam_weight"] * 0.1
    
    # 6. Energy match (10% weight)
    if inputs["is_peak_hour"] and inputs["is_hard_subject"]:
        score += 10  # Perfect match: hard subject at peak time
    elif not inputs["is_peak_hour"] and inputs["is_hard_subject"]:
        score -= 5   # Penalty: hard subject at low energy
    
    return min(100, max(0, score))
```

### ADHD-Specific Rules

```python
def apply_adhd_rules(plan, energy_profile, session_history):
    """Post-processing rules specifically for ADHD"""
    
    # Rule 1: Never more than 3 sessions
    if len(plan) > 3:
        plan = plan[:3]
    
    # Rule 2: Hardest subject first (when energy is highest)
    plan.sort(key=lambda s: s.difficulty, reverse=True)
    
    # Rule 3: If Monday, reduce expectations
    if today_is_monday():
        plan = plan[:2]  # Only 2 sessions on Mondays
    
    # Rule 4: If crashed (afternoon + low recent scores), only easy subjects
    if now_is_crash_hour(energy_profile):
        plan = [s for s in plan if s.difficulty <= 4]
        if not plan:
            plan = [StudySession("Descanso", "15 min sin pantalla")]
    
    # Rule 5: If studied >4h today already, suggest stopping
    if hours_studied_today(session_history) >= 4:
        plan = [StudySession("Ya estudiaste bastante", "Descansá. Mañana seguimos.")]
    
    return plan
```

---

## 13. Anti-Pattern Detection

The agent memory system learns patterns over time. For ADHD, specific anti-patterns to detect:

### Detection Logic

```python
ADHD_ANTIPATTERNS = {
    "monday_procrastination": {
        "description": "Always less productive on Mondays",
        "detect": lambda stats: stats.by_day_of_week["Monday"] < stats.average * 0.5,
        "response": "Los lunes son tu día difícil. Planificamos solo 1 sesión corta hoy."
    },
    
    "hyperfocus_trap": {
        "description": "Sessions exceeding 2 hours without breaks",
        "detect": lambda sessions: any(s.duration > 120 for s in sessions[-10:]),
        "response": "Llevás más de 2 horas. Tu cerebro ya absorbe menos. 15 min de descanso."
    },
    
    "topic_avoidance": {
        "description": "Consistently skipping a specific subject",
        "detect": lambda stats: any(
            s.sessions_last_week == 0 and s.days_until_exam < 14
            for s in stats.subjects
        ),
        "response": "Llevás {days} días sin tocar {subject}. ¿10 minutos nomás?"
    },
    
    "night_study_inefficiency": {
        "description": "Quiz scores drop significantly after 10pm",
        "detect": lambda stats: stats.night_quiz_avg < stats.day_quiz_avg * 0.6,
        "response": "Tu tasa de acierto de noche es {night}% vs {day}% de día. Mejor descansá."
    },
    
    "topic_switching": {
        "description": "Constantly switching between subjects in one session",
        "detect": lambda session: session.topic_changes > 3,
        "response": "Cambiaste de tema {n} veces. Eso es normal con TDAH. Elegí UNO y quedate."
    },
    
    "weekend_gap": {
        "description": "Never studying on weekends",
        "detect": lambda stats: stats.by_day_of_week["Saturday"] == 0 
            and stats.by_day_of_week["Sunday"] == 0,
        "response": "No estudiás los findes. ¿Querés que planifique 30 min el sábado?"
    },
    
    "perfectionism_loop": {
        "description": "Re-reading the same material without progressing",
        "detect": lambda sessions: len(set(s.topic for s in sessions[-5:])) == 1,
        "response": "Llevás 5 sesiones del mismo tema. ¿Avanzamos al siguiente o necesitás ayuda?"
    }
}
```

---

## 14. UX Principles for ADHD

Based on research and community feedback (r/ADHD, Obsidian forums, academic studies):

### DO

1. **Show only today** — The semester view is overwhelming. Today has 2-3 items. That's it.
2. **Decide for the user** — "Study X" not "What do you want to study?"
3. **Use body-friendly language** — "Let's start with..." not "You should..."
4. **Celebrate small wins** — "3 sessions done today. That's solid." not "You're behind."
5. **Make failure graceful** — Missed yesterday? "No pasa nada. Hoy seguimos con..."
6. **Default to micro-tasks** — "Read 2 pages" not "Study Chapter 5"
7. **Use visual progress** — Progress bars, checkmarks, percentage completion
8. **Respect energy levels** — Don't suggest hard tasks at 2pm

### DON'T

1. **Don't show the full semester plan** — Paralysis trigger
2. **Don't use guilt language** — "You missed 3 days" → shame → avoidance
3. **Don't ask open-ended questions** — "What do you want to study?" → decision paralysis
4. **Don't overwhelm with options** — 3 choices max, ideally 1
5. **Don't require daily maintenance** — The system runs itself
6. **Don't punish inconsistency** — ADHD is inherently inconsistent. Adapt to it.
7. **Don't assume neurotypical patterns** — "Study every day for 2 hours" doesn't work

### Obsidian-Specific UX

| Element | Implementation |
|---|---|
| HOY.md | Top of file explorer, always visible |
| Checkboxes | `- [ ]` tasks that can be checked off |
| Callouts | `> [!tip]` for suggestions, `> [!warning]` for anti-patterns |
| Links | `[[Topic]]` links for quick navigation |
| Tags | `#hoy` `#repaso` `#examen-proximo` for filtering |
| Dataview | Auto-generated tables from frontmatter |

---

## 15. Data Flow & Integration Patterns

### Daily Automated Flow

```
08:00 — Cron job triggers gemini hoy
         ↓
    Agent reads:
      - Config/Materias.md (subjects, deadlines)
      - Config/Mi energía.md (peak hours)
      - crab-engram: mem_reviews (what to review)
      - crab-engram: mem_stats (mastery levels)
         ↓
    Agent calculates priority → writes HOY.md
         ↓
    Student opens Obsidian → sees plan

09:00 — Student starts studying
         ↓
    Opens HOY.md → clicks link to Algoritmos/DP
    Reads material, or runs: gemini test
         ↓
    notebooklm generates quiz → student answers
    Agent scores → mem_save → mastery updated
         ↓
    25 min timer → break → next session

21:00 — Student says: gemini resumen del día
         ↓
    Agent queries crab-engram for today's sessions
    Writes daily summary in Obsidian
    Updates Progreso.md
    Updates Repaso.md with tomorrow's reviews
```

### Weekly Automated Flow

```
Domingo 20:00 — Cron job triggers gemini semana
         ↓
    Agent reads all subject deadlines
    crab-engram: mem_antipatterns
         ↓
    Generates SEMANA.md:
      - Total study hours target
      - Distribution across days
      - Warning about known anti-patterns
      - Upcoming exams
```

### Manual Interactions

| When | Student Says | Agent Does |
|---|---|---|
| Starting the day | `gemini hoy` | Generates HOY.md |
| Want a quiz | `gemini test [topic]` | notebooklm → quiz → score → save |
| Need review | `gemini repaso` | crab-engram → spaced items → present |
| Can't focus | `gemini no-puedo` | 1 micro-task or break suggestion |
| Check progress | `gemini progreso` | crab-engram stats → Progreso.md |
| Summary | `gemini resumen [topic]` | notebooklm → summary → save to vault |
| End of day | `gemini resumen del día` | Session summary → daily log |

---

## 16. Implementation Blueprint

### Phase 1: Foundation (Week 1)

- [ ] Install Gemini CLI: `npm install -g @google/gemini-cli`
- [ ] Build notebooklm-rust-mcp: `cargo build --release`
- [ ] Build the-crab-engram: `cargo build --release`
- [ ] Configure MCP servers in `~/.gemini/settings.json`
- [ ] Create Obsidian vault structure
- [ ] Write GEMINI.md (agent personality)
- [ ] Create Config/Materias.md with real subjects
- [ ] Create Config/Mi energía.md with personal energy profile

### Phase 2: Daily Flow (Week 2)

- [ ] Test `gemini hoy` — does it generate a reasonable plan?
- [ ] Test `gemini test` — does notebooklm generate good quizzes?
- [ ] Set up cron job for automatic HOY.md generation
- [ ] Create HOY.md template with Templater
- [ ] Test full flow: plan → study → quiz → score → save

### Phase 3: Memory & Repetition (Week 3)

- [ ] Verify crab-engram saves sessions correctly
- [ ] Test `mem_reviews` — does spaced repetition work?
- [ ] Test `mem_stats` — does mastery tracking work?
- [ ] Create Progreso.md dashboard
- [ ] Create Repaso.md with daily review items
- [ ] Set up anti-pattern detection

### Phase 4: Polish (Week 4)

- [ ] Refine priority algorithm based on real usage
- [ ] Add Obsidian Dataview queries for auto-dashboards
- [ ] Create weekly summary automation
- [ ] Add session logging to Sesiones/ folder
- [ ] Test "no-puedo" mode — does it reduce overwhelm?
- [ ] Iterate on GEMINI.md based on what works

---

## 17. Appendix: Research Sources

### Academic Research
- Barkley, R. (1997). "Behavioral inhibition, sustained attention, and executive functions." *Psychological Bulletin*, 121(1), 65-94.
- Sweller, J. (1988). "Cognitive load during problem solving." *Cognitive Science*, 12(2), 257-285.
- Ebbinghaus, H. (1885). *Memory: A Contribution to Experimental Psychology.*
- Jarrett Ye (2023). FSRS: Free Spaced Repetition Scheduler. open-spaced-repetition/fsrs4anki.
- PMC6406620: "Strategies for Coping with Time-Related and Productivity Challenges of LD/ADHD"
- MDPI (2025). "ADHD in Adulthood: Clinical Presentation, Comorbidities, and Compensatory Strategies"
- Frontiers in Psychology (2025). "Evaluating attention deficit and hyperactivity disorder: compensatory strategies"

### Tools & Protocols
- Google NotebookLM: notebooklm.google.com
- MCP (Model Context Protocol): modelcontextprotocol.io
- Gemini CLI: github.com/google-gemini/gemini-cli
- Obsidian: obsidian.md
- Obsidian Local REST API: github.com/coddingtonbear/obsidian-local-rest-api

### Community Resources
- r/ADHD — Executive function strategies
- r/ObsidianMD — Academic workflows
- r/notebooklm — Study workflows
- r/Anki — Spaced repetition algorithms (FSRS vs SM-2)
- Obsidian Forum — "A maintainable second brain for someone with ADHD" (forum.obsidian.md/t/36106)
