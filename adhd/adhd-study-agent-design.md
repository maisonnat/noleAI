# ADHD University Agent — Ecosystem Design
## NotebookLM + Crab Engram + Obsidian

**Context**: University student with ADHD (37). Needs a system that thinks FOR them,
not just WITH them. Three existing Rust projects form the foundation.

---

## The Three Pillars

```
┌─────────────────────────────────────────────────────────────┐
│                        OBSIDIAN                              │
│              (Your Second Brain / Interface)                 │
│                                                              │
│  📁 Universidad/                                            │
│  ├── 📁 Algoritmos/                                        │
│  │   ├── 📝 Notas de clase.md                              │
│  │   ├── 📝 DP Patterns.md          ← auto-synced from     │
│  │   ├── 📝 Quiz Results.md          NotebookLM + Engram   │
│  │   └── 📝 Exam Prep.md                                    │
│  ├── 📁 Álgebra Lineal/                                    │
│  │   └── ...                                                │
│  ├── 📋 HOY.md                     ← auto-generated daily  │
│  ├── 📊 Progreso.md                ← mastery dashboard     │
│  └── 🧠 Repaso.md                  ← spaced repetition     │
│                                                              │
└──────────────┬──────────────────┬───────────────────────────┘
               │                  │
               ▼                  ▼
┌──────────────────────┐  ┌──────────────────────────┐
│  notebooklm-rust-mcp │  │    the-crab-engram 🦀    │
│                      │  │                          │
│  INGESTA             │  │  MEMORIA PERSISTENTE     │
│                      │  │                          │
│  • Upload PDFs       │  │  • Qué estudiaste        │
│  • Generar quizzes   │  │  • Qué sabes (beliefs)   │
│  • Generar flashcards│  │  • Spaced repetition     │
│  • Resumir temas     │  │  • Graph de conexiones   │
│  • Deep research     │  │  • Anti-patterns de      │
│  • Audio summaries   │  │    estudio (procrastinas │
│  • Mind maps         │  │    los lunes?)           │
│                      │  │  • Consolidation         │
│  "El profesor"       │  │    (relaciona temas)     │
│                      │  │                          │
│  Fuente: Google      │  │  "Tu diario de estudio"  │
│  NotebookLM          │  │                          │
└──────────────────────┘  └──────────────────────────┘
               │                  │
               └────────┬─────────┘
                        │
                        ▼
         ┌──────────────────────────┐
         │   MCP Orchestrator       │
         │   (Claude / Ollama)      │
         │                          │
         │   "Tu coach de estudio"  │
         │                          │
         │   • Prioriza materias    │
         │   • Planifica el día     │
         │   • Detecta overwhelm    │
         │   • Genera Obsidian notes│
         │   • Coordina ambos MCP   │
         └──────────────────────────┘
```

---

## Why This Combination is Powerful for ADHD

### notebooklm-rust-mcp → Alivia la carga cognitiva de CREAR material

ADHD no es "no poder estudiar" — es "no poder decidir QUÉ estudiar y CÓMO empezar".

| Sin herramienta | Con NotebookLM MCP |
|----------------|-------------------|
| Abro el PDF de 300 páginas → parálisis | `source_add_file` + `artifact_generate(quiz)` → ya tengo 5 preguntas |
| "¿Qué es un autómata?" → 20 min de Google | `ask_question` → respuesta con cita del PDF |
| Hacer flashcards manualmente → procrastinación eterna | `artifact_generate(flashcards)` → listas en 10 segundos |
| Resumir capítulo → tarea interminable | `notebook_summary` → resumen + temas sugeridos |

### the-crab-engram → Alivia la carga cognitiva de RECORDAR

| Sin herramienta | Con Crab Engram |
|----------------|----------------|
| "¿Ya estudié esto?" → no sé | `mem_search` → "Sí, el martes. Sacaste 3/5 en el quiz." |
| "¿Qué necesito repasar?" → adivino | `mem_reviews` → spaced repetition calculado |
| "¿Cómo se conecta esto con lo otro?" → lo olvido | `mem_graph` → visualiza conexiones entre temas |
| Estudio 3 horas → no sé cuánto avanzo | `mem_stats` → "12 sesiones, 45% mastery en DP" |
| Repito los mismos errores | `mem_antipatterns` → "Siempre fallas en recursión cuando estudias después de las 3pm" |

### Obsidian → El lugar donde TODO vive

No necesitas una app nueva. Todo aparece en tu vault de Obsidian:

- Notas de clase → las escribes tú
- Resúmenes de NotebookLM → se sincronizan automáticamente
- Quiz results → se guardan como frontmatter
- Plan del día → se genera como nota
- Repaso programado → aparece como callout

---

## Obsidian Vault Structure

```
Universidad/
│
├── 📋 HOY.md                          ← Auto-generado cada mañana
│   ---
│   date: 2026-04-15
│   energy_level: high
│   total_planned: 1h 50min
│   ---
│   ## Prioridades de hoy
│   1. [[Algoritmos/DP Patterns|Algoritmos — DP]] (25 min)
│      - Repasar los 3 patrones
│      - Hacer quiz de 5 preguntas
│   2. [[Álgebra/Valores Propios|Álgebra — Eigenvalues]] (25 min)
│      - Ver segmento de clase
│      - Tomar notas
│   3. [[Historia/Cap5|Historia — Cap 5]] (25 min)
│      - Lectura ligera, sin presión
│
├── 📊 Progreso.md                     ← Auto-actualizado
│   ---
│   last_updated: 2026-04-15
│   ---
│   | Materia | Mastery | Última sesión | Próximo examen |
│   |---------|---------|---------------|----------------|
│   | Algoritmos | ████░░░░ 45% | Hoy | 5 días |
│   | Álgebra | ███░░░░░ 35% | Ayer | 12 días |
│   | Historia | ██████░░ 65% | Hace 3 días | 20 días |
│
├── 🧠 Repaso.md                       ← Spaced repetition
│   ## Para hoy (15 abril)
│   - [ ] [[Algoritmos/DP#Patrones|DP: Memoization vs Tabulation]]
│     - Último repaso: hace 3 días. Confianza: baja.
│   - [ ] [[Álgebra/Eigenvalues#Definición|Eigenvalues: definición]]
│     - Último repaso: hace 5 días. Confianza: media.
│
├── 📁 Algoritmos/
│   ├── 📝 Notas clase 1-8.md
│   ├── 📝 DP Patterns.md              ← Contenido de NotebookLM
│   │   ---
│   │   notebooklm_source: true
│   │   mastery: 45
│   │   confusion: [subsequence, knapsack]
│   │   ---
│   │   ## Los 3 patrones de DP
│   │   1. **Memoization** (top-down)...
│   │   2. **Tabulation** (bottom-up)...
│   │   3. **Space-optimized**...
│   │
│   ├── 📝 Quiz Results.md
│   │   ## 2026-04-15 — Quiz de DP
│   │   - 3/5 correctas
│   │   - ❌ Fallaste: knapsack 0/1
│   │   - ❌ Fallaste: longest common subsequence
│   │   - ✅ Correcto: fibonacci
│   │   - ✅ Correcto: coin change
│   │   - ✅ Correcto: edit distance
│   │
│   └── 📝 Exam Prep.md
│       ---
│       exam_date: 2026-05-20
│       weight: 30%
│       ---
│       ## Temas a dominar
│       - [x] Sorting algorithms
│       - [ ] Dynamic programming ← focus esta semana
│       - [ ] Graph algorithms
│       - [ ] Greedy algorithms
│
├── 📁 Álgebra Lineal/
│   └── ...
│
├── 📁 Sesiones/                       ← Log de cada estudio
│   ├── 2026-04-15-0900-algoritmos.md
│   │   ---
│   │   duration: 25min
│   │   subject: Algoritmos
│   │   topic: DP Patterns
│   │   focus_quality: 8/10
│   │   notes_taken: true
│   │   ---
│   │   ## Qué hice
│   │   - Revise memoization vs tabulation
│   │   - Hice quiz, saque 3/5
│   │   - Me confundí en knapsack
│   │
│   └── ...
│
└── 📁 Config/
    ├── 📝 Materias.md                 ← Definición de cada materia
    │   ## Algoritmos
    │   - exam_date: 2026-05-20
    │   - weight: 30%
    │   - difficulty: 8/10
    │   - professor_uploads: Monday
    │   - office_hours: Tuesday 2-4pm
    │   - notebooklm_id: uuid-here
    │
    ├── 📝 Mi energía.md               ← Perfil de TDAH
    │   ## Cuándo soy más productivo
    │   - Peak: 9-11am (materias difíciles aquí)
    │   - Crash: 2-3pm (solo review fácil)
    │   - Recovery: 4-6pm (materias medianas)
    │   ## Cuánto puedo concentrarme
    │   - Max: 25 min (Pomodoro)
    │   - Break: 5 min
    │   - Long break: cada 4 sesiones, 15 min
    │   ## Qué me distrae
    │   - Ver el semestre completo → overwhelm
    │   - Más de 3 pestañas abiertas → parálisis
    │   - Estudiar en la cama → me duermo
    │
    └── 📝 Atajos.md
        ## Comandos del agente
        - "¿Qué estudio?" → genera plan del día
        - "No puedo" → modo overwhelm (1 tarea)
        - "Test me" → genera quiz del tema actual
        - "¿Qué me falta?" → mastery gap analysis
        - "Repaso" → spaced repetition de hoy
        - "¿Cómo voy?" → dashboard de progreso
```

---

## Data Flow: How It All Connects

### Daily Flow

```
08:00  El agente genera HOY.md
       ├── Lee materias desde Config/Materias.md
       ├── Consulta Crab Engram: mem_reviews (qué repasar)
       ├── Consulta Crab Engram: mem_stats (mastery actual)
       ├── Ejecuta algoritmo de prioridad
       ├── Genera plan de 2-3 sesiones
       └── Escribe HOY.md en Obsidian

09:00  Empezás a estudiar
       ├── Abrís HOY.md → ves "Algoritmos — DP Patterns"
       ├── El agente: "Querés que te haga un quiz antes de empezar?"
       ├── Vos: "Sí"
       ├── Agente → NotebookLM MCP: artifact_generate(quiz, DP)
       ├── NotebookLM genera 5 preguntas
       ├── Las hacés en Obsidian (callout blocks)
       ├── Resultados → Crab Engram: mem_save(quiz_result)
       └── Mastery se actualiza automáticamente

09:25  Timer termina
       ├── Agente: "5 minutos de descanso. Después: Álgebra Lineal."
       ├── Crab Engram: mem_session_end → guarda sesión
       └── Obsidian: se escribe Sesiones/2026-04-15-0900-algoritmos.md

14:00  Crash de energía
       ├── Vos: "No puedo más"
       ├── Agente detecta hora + perfil de energía
       ├── "Es tu crash de las 2pm. Opciones:
       │    1. Review ligero de Historia (tu materia fácil)
       │    2. 15 min de descanso real (sin pantalla)
       │    3. Flashcards de algo que ya dominás (refuerzo)"
       └── Vos elegís → agente adapta

21:00  Antes de dormir
       ├── Agente: "¿Querés un resumen del día?"
       ├── Crab Engram: mem_session_summary
       ├── Genera resumen en Obsidian: Sesiones/daily-summary-2026-04-15.md
       └── "Hoy estudiaste 1h50min. 3 sesiones. 
            Avanzaste en DP y Eigenvalues. 
            Mañana: seguís con DP + empezás Graph Algorithms."
```

### Weekly Flow

```
Domingo noche:
  Agente genera SEMANA.md
  ├── Revisa exámenes próximos
  ├── Revisa Crab Engram: antipatterns
  │   ("Los lunes siempre procrastinás — planifica con cuidado")
  ├── Revisa mastery gaps
  ├── Distribuye sesiones de la semana
  └── "Esta semana necesitás 10 horas de estudio.
       Te las reparto así: Lun 1.5h, Mar 2h, Mié 1.5h, 
       Jue 2h, Vie 1.5h, Sáb 1.5h, Dom descanso."
```

---

## MCP Configuration

```json
{
  "mcpServers": {
    "notebooklm": {
      "command": "/path/to/notebooklm-mcp",
      "args": []
    },
    "crab-engram": {
      "command": "/path/to/the-crab-engram",
      "args": ["mcp"]
    },
    "obsidian": {
      "command": "npx",
      "args": ["-y", "mcp-obsidian"],
      "env": {
        "OBSIDIAN_API_KEY": "your-key",
        "OBSIDIAN_VAULT_PATH": "/path/to/vault"
      }
    }
  }
}
```

Con esta config, Claude/Cursor/OpenClaw puede:
- Leer y escribir notas en Obsidian (mcp-obsidian)
- Crear quizzes en NotebookLM (notebooklm-rust-mcp)
- Guardar y recuperar memoria (the-crab-engram)

---

## Anti-Patterns Detection (Specifically for ADHD Study)

Crab Engram tiene `mem_antipatterns`. Se puede configurar para detectar patrones de estudio específicos de TDAH:

```
Patrones que el sistema aprende:

1. "Procrastinación del lunes"
   → Si los lunes tenés 50% menos sesiones que otros días
   → Sugerencia: "Lunes es tu día difícil. Planifiquemos 1 sola 
     sesión corta en vez de 3."

2. "Hiperfoco destructivo"
   → Si una sesión duró >2 horas sin break
   → Alerta: "Llevás 2h15min sin parar. Tu cerebro ya no retiene. 
     Descansá 15 min."

3. "Evitación por dificultad"
   → Si siempre posponés una materia pero no otra
   → Sugerencia: "Llevás 3 días sin tocar Álgebra. ¿Es porque es 
     difícil? Podemos hacer solo 10 min de review."

4. "Estudio nocturno improductivo"
   → Si después de las 10pm la calidad de respuestas baja
   → Sugerencia: "Tu tasa de acierto en quizzes después de las 
     10pm es 40%. Mejor descansá y empezá mañana."

5. "Cambio constante de tema"
   → Si en una sesión saltás entre 3+ materias sin completar
   → "Noté que cambiaste de tema 3 veces. Eso es normal con TDAH. 
     ¿Querés que te dé UNA sola tarea pequeña?"
```

---

## Technology Summary

| Componente | Proyecto | Qué resuelve para TDAH |
|-----------|----------|----------------------|
| Ingesta de material | notebooklm-rust-mcp | "No tengo que hacer flashcards a mano" |
| Memoria persistente | the-crab-engram | "No tengo que recordar qué estudié" |
| Spaced repetition | crab-engram `mem_reviews` | "No tengo que decidir qué repasar" |
| Anti-patterns | crab-engram `mem_antipatterns` | "El sistema sabe cuándo procrastino" |
| Graph de conocimiento | crab-engram `mem_graph` | "Veo cómo se conectan los temas" |
| Notas y planificación | Obsidian | "Todo está en UN solo lugar" |
| Priorización | Agente (Claude/Ollama) | "No tengo que decidir qué estudiar" |
| Interfaz | Obsidian | "Ya la conozco, no necesito app nueva" |

Los dos proyectos Rust son la infraestructura. Obsidian es la interfaz. El agente es el cerebro que coordina.
