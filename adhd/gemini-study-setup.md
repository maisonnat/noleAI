# Setup: Gemini CLI + NotebookLM MCP + Crab Engram + Obsidian

## Tu Stack Definitiva

```
┌─────────────────────────────────────────────────────┐
│                    OBSIDIAN                          │
│            (Tu segundo cerebro — donde vivís)        │
│                                                      │
│  Escribís notas, ves el plan, hacés quizzes,        │
│  revisás progreso. TODO acá.                         │
│                                                      │
│  Necesitás algo?                                     │
│  → Abrís terminal → gemini                           │
│  → O usás el plugin de Obsidian para Gemini          │
│                                                      │
└────────────────────┬────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────┐
│                  GEMINI CLI                          │
│            (Tu coach — siempre gratis)               │
│                                                      │
│  • 60 req/min, 1000 req/día — gratis con Google     │
│  • 1M token context window                           │
│  • Conecta a MCP servers                             │
│  • Lee/escribe archivos en tu vault                  │
│  • GEMINI.md = su "memoria" de cómo ayudarte         │
│                                                      │
│  Config: ~/.gemini/settings.json                     │
│                                                      │
└────────┬────────────────────────┬───────────────────┘
         │                        │
         ▼                        ▼
┌──────────────────┐    ┌──────────────────────┐
│ notebooklm-rust  │    │   the-crab-engram    │
│ -mcp             │    │       🦀             │
│                  │    │                      │
│ Genera material: │    │ Recuerda todo:       │
│ • Quizzes        │    │ • Sesiones pasadas   │
│ • Flashcards     │    │ • Mastery levels     │
│ • Resúmenes      │    │ • Spaced repetition  │
│ • Audio          │    │ • Anti-patterns      │
│ • Deep research  │    │ • Graph edges        │
│ • Mind maps      │    │ • Beliefs            │
└──────────────────┘    └──────────────────────┘
```

---

## Paso 1: Instalar Gemini CLI

```bash
npm install -g @google/gemini-cli

# Autenticarte con tu cuenta Google (la misma de NotebookLM)
gemini
# Te abre el browser → login con Google
```

**Gratis**: 60 requests/minuto, 1000/día. Para un estudiante es más que suficiente.

---

## Paso 2: Configurar MCP Servers

`~/.gemini/settings.json`:

```json
{
  "theme": "Default",
  "selectedAuthType": "oauth-personal",
  "mcpServers": {
    "notebooklm": {
      "command": "/path/to/notebooklm-mcp",
      "args": [],
      "trust": true
    },
    "crab-engram": {
      "command": "/path/to/the-crab-engram",
      "args": ["mcp"],
      "trust": true
    },
    "obsidian": {
      "command": "npx",
      "args": ["-y", "mcp-obsidian"],
      "env": {
        "OBSIDIAN_API_KEY": "tu-api-key-de-obsidian",
        "OBSIDIAN_VAULT_PATH": "/path/to/tu-vault"
      },
      "trust": true
    }
  }
}
```

**Nota sobre mcp-obsidian**: Necesitás el plugin [Local REST API](https://github.com/coddingtonbear/obsidian-local-rest-api) en Obsidian. Te da una API key para leer/escribir notas.

---

## Paso 3: GEMINI.md — La "Personalidad" del Agente

Creás un archivo `GEMINI.md` en la raíz de tu vault de Obsidian. Gemini CLI lo lee como contexto persistente.

`/path/to/obsidian-vault/GEMINI.md`:

```markdown
# Study Coach — ADHD University Assistant

## Who You Are
You are a study coach for a university student with ADHD (age 37).
Your job is to reduce cognitive load: decide what to study, when, and how.
The student should spend minimal time planning and maximal time learning.

## Your Tools
- **notebooklm** (MCP): Generate quizzes, flashcards, summaries from course PDFs.
  Use `ask_question` to explain topics, `artifact_generate` for quizzes/flashcards.
- **crab-engram** (MCP): Persistent memory of study sessions, mastery levels,
  spaced repetition schedule. Use `mem_save`, `mem_search`, `mem_reviews`.
- **file system**: Read/write Obsidian vault notes directly.

## Vault Structure
The student's university notes live in:
  /path/to/obsidian-vault/Universidad/

Key files:
  - HOY.md — daily plan (you generate this)
  - Progreso.md — mastery dashboard (you update this)
  - Repaso.md — spaced repetition for today
  - Config/Materias.md — subject definitions
  - Config/Mi energía.md — ADHD energy profile
  - {Subject}/ — per-subject notes, quizzes, exam prep

## How to Behave
1. **Be direct**: No fluff. "Study DP for 25 min" not "I think you might consider..."
2. **Max 3 items**: Never show more than 3 tasks at once. The rest doesn't exist yet.
3. **Decide for them**: "Start with Algorithms" not "What would you like to study?"
4. **Energy-aware**: Check Config/Mi energía.md. Hard subjects at peak hours only.
5. **Graceful failure**: Missed yesterday? Recalculate. No guilt.
6. **ADHD patterns**: Watch for overwhelm, hyperfocus, procrastination cycles.
   Record anti-patterns in crab-engram with `mem_save`.

## Commands the Student Will Use
- "¿Qué estudio?" → Generate today's plan, write HOY.md
- "No puedo" → Overwhelm mode: 1 tiny task or suggest break
- "Test me [topic]" → Generate quiz via notebooklm, score it, save to engram
- "¿Qué me falta?" → Mastery gap analysis from engram stats
- "Repaso" → Get spaced repetition items from engram, present them
- "Resumen [topic]" → Get summary from notebooklm, save to vault
- "¿Cómo voy?" → Dashboard from engram stats, write Progreso.md

## Session Flow
1. Read Config/Materias.md (subjects, deadlines, weights)
2. Read Config/Mi energía.md (peak hours, crash hours)
3. Query crab-engram: mem_reviews (what to review today)
4. Query crab-engram: mem_stats (current mastery levels)
5. Calculate priority (deadline urgency + mastery gap + energy match)
6. Write HOY.md with 2-3 sessions
7. When student finishes a session:
   - Save session to crab-engram: mem_save
   - Update mastery if quiz was taken
   - Update Repaso.md with next review date

## Anti-Patterns to Watch For
- Monday procrastination (record in engram if detected)
- Sessions >2h without break (hyperfocus alert)
- Constant topic switching (suggest single focus)
- Late-night study with low quiz scores (suggest rest)
- Avoiding specific subjects (suggest 10-min micro-session)
```

---

## Paso 4: Comandos Diarios

### Desde la terminal (en la raíz del vault)

```bash
# ¿Qué estudio hoy?
cd ~/Obsidian/Universidad
gemini -p "¿Qué estudio hoy? Genera HOY.md"

# Test rápido
gemini -p "Test me on dynamic programming"

# No puedo concentrarme
gemini -p "No puedo concentrarme, estoy en modo crash"

# ¿Cómo voy?
gemini -p "¿Cómo voy? Actualiza Progreso.md"

# Repaso
gemini -p "¿Qué tengo que repasar hoy?"

# Resumen de un tema
gemini -p "Resumí el capítulo de eigenvalues, guardalo en Álgebra Lineal/"

# Antes de dormir
gemini -p "Resumen del día. ¿Qué logré? ¿Qué pendiente queda?"
```

### Desde Obsidian (más integrado)

**Opción A: Terminal embebida**
Obsidian tiene un plugin [Terminal](https://github.com/polyipseity/obsidian-shell-commands) que puede ejecutar comandos shell. Configurás un botón que ejecuta `gemini -p "..."`.

**Opción B: Plugin de comandos personalizados**
```bash
# ~/.gemini/settings.json — custom commands
{
  "customCommands": {
    "hoy": {
      "prompt": "Lee Config/Materias.md, consultá crab-engram para reviews y stats, generá HOY.md con máximo 3 sesiones priorizadas.",
      "description": "Generar plan del día"
    },
    "test": {
      "prompt": "Pedí a notebooklm que genere un quiz de 5 preguntas del tema que el estudiante mencione. Después calificá las respuestas y guardá el resultado en crab-engram.",
      "description": "Generar y calificar quiz"
    },
    "repaso": {
      "prompt": "Consultá crab-engram mem_reviews para ver qué hay que repasar hoy. Presentá como lista de checkboxes en formato Obsidian.",
      "description": "Spaced repetition de hoy"
    },
    "resumen": {
      "prompt": "Pedí a notebooklm un resumen del tema mencionado. Guardalo como nota en la carpeta de la materia correspondiente.",
      "description": "Resumir tema"
    },
    "no-puedo": {
      "prompt": "Modo overwhelm. Ofrecé UNA tarea de máximo 10 minutos O un descanso de 15 minutos. Nada más.",
      "description": "Modo sobrecarga"
    }
  }
}
```

Entonces:
```bash
gemini hoy        # Genera plan del día
gemini test       # Pide tema y genera quiz
gemini repaso     # Muestra qué repasar hoy
gemini no-puedo   # Modo overwhelm
```

**Opción C: Cron job para HOY.md automático**

```bash
# Cada mañana a las 8am, genera el plan del día
crontab -e
0 8 * * * cd ~/Obsidian/Universidad && gemini -p "Genera HOY.md para hoy" --output-format json > /dev/null 2>&1
```

---

## Flujo Real de un Día

```
08:00  Abrís Obsidian → ves HOY.md ya generado
       "Hoy: 1) Algoritmos DP 25min, 2) Álgebra eigenvalues 25min, 3) Historia review 25min"

09:00  Abrís HOY.md → leés "Algoritmos — DP Patterns"
       Abrís terminal en Obsidian:
       > gemini test
       "¿Sobre qué tema?" → "DP"
       NotebookLM genera 5 preguntas → las hacés en Obsidian
       Gemini califica → guarda en crab-engram
       Crab-engram actualiza mastery: 45% → 52%

09:25  Timer suena.
       > gemini repaso
       "Mañana repasás: Eigenvalues definición (último repaso hace 3 días)"

14:00  Crash. Cerebro apagado.
       > gemini no-puedo
       "Opción: Leé 2 páginas de Historia (tu materia fácil). 5 minutos. Sin presión.
        O descansá 15 min sin pantalla."

21:00  Antes de dormir.
       > gemini resumen del día
       Gemini escribe en Obsidian: Sesiones/2026-04-15-resumen.md
       "Hoy: 1h50min estudiados. 3 sesiones. DP mejoró 7 puntos.
        Mañana: seguís con DP (quiz de repaso) + empezás Graph Algorithms."
```

---

## Ventajas de Gemini CLI vs Otros

| | Gemini CLI | Claude Code | Cursor |
|---|---|---|---|
| **Precio** | Gratis (1000/día) | $20/mes | $20/mes |
| **MCP** | ✅ | ✅ | ✅ |
| **Contexto** | 1M tokens | 200K | 128K |
| **Ecosistema** | Google (NotebookLM nativo) | Anthropic | OpenAI |
| **Scriptable** | ✅ headless mode | ✅ | ❌ |
| **Custom commands** | ✅ GEMINI.md | ✅ CLAUDE.md | ✅ |
| **Terminal first** | ✅ | ✅ | ❌ (IDE) |

Para tu caso: **Gemini CLI es la mejor opción** porque:
1. Gratis — sos estudiante
2. 1M context — puede leer todo tu vault de una
3. NotebookLM es de Google — integración natural
4. Headless mode — podés automatizar desde cron
5. Ya estás en Google (NotebookLM, Drive, etc.)

---

## Archivos a Crear

```
~/Obsidian/Universidad/
├── GEMINI.md                    ← Contexto del agente
├── 📋 HOY.md                    ← Auto-generado
├── 📊 Progreso.md               ← Auto-actualizado
├── 🧠 Repaso.md                 ← Spaced repetition
├── Config/
│   ├── Materias.md              ← Tus materias
│   └── Mi energía.md            ← Tu perfil TDAH
├── Algoritmos/
│   ├── Notas/
│   ├── Quiz Results/
│   └── Exam Prep.md
├── Álgebra Lineal/
│   └── ...
└── Sesiones/
    └── daily-summaries/
```

¿Querés que te genere los templates de cada archivo (Materias.md, Mi energía.md, HOY.md, etc.) para que los copies directo a tu vault?
