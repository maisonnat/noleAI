# NoleAI: Lista de Tareas (Backlog)

## 📋 Hito 1: Setup e Infraestructura
- [X] Crear estructura de `Cargo workspace`.
- [X] Integrar `the-crab-engram` como dependencia local.
- [X] Integrar `notebooklm-rust-mcp` como dependencia local.
- [X] Definir el trait `EventPublisher` para el bus de mensajes.
- [X] Crear la estructura de carpetas inicial en el Vault de Obsidian.

## 📋 Hito 2: Sincronización con Obsidian
- [X] Implementar parser de `Config/Materias.md` (formato YAML/Markdown).
- [X] Crear el generador de `HOY.md` con lógica de prioridad básica.
- [X] Crear el actualizador de `Progreso.md` basado en mock-data de maestría.
- [X] Implementar un `Watcher` simple que detecte cambios en el vault.

## 📋 Hito 3: Tauri HUD (V1)
- [X] Setup de Tauri con Rust (backend) y Framework ligero (frontend).
- [X] Crear ventana flotante sin bordes para el Pomodoro.
- [X] Implementar comunicación `Tauri -> Core` para iniciar/pausar sesiones.
- [X] Mostrar la tarea actual extraída de `HOY.md` en el HUD.

## 📋 Hito 4: Memoria e Inteligencia
- [X] Conectar el fin de sesión del Timer con `mem_save` de Engram.
- [X] Implementar comando `test [topic]` que dispare el flujo de NotebookLM.
- [X] Crear la UI de quiz en Tauri para responder preguntas sin salir del HUD.
- [X] Automatizar la generación de `Repaso.md` usando `mem_reviews`.

## 📋 Hito 5: Refinamiento TDAH
- [X] Implementar el "Modo Sobrecarga" (UI simplificada).
- [X] Añadir lógica de "Andamiaje Socrático" en los prompts de IA para Matemáticas.
- [X] Configurar alertas de anti-patrones (ej: sesión muy larga sin break).
- [X] Generar resumen diario automático en `Sesiones/daily-summaries/`.

## 📋 Hito 6: Ciberdefensa Ops
- [X] Crear plantillas específicas para toma de notas de Ciberdefensa.
- [X] Integrar visualizador de grafos de conocimiento de Engram en el HUD.
- [X] Pruebas finales de estrés con volumen real de PDFs de FADENA.
