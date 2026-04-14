# NoleAI: Lista de Tareas (Backlog)

## 📋 Hito 1: Setup e Infraestructura
- [ ] Crear estructura de `Cargo workspace`.
- [ ] Integrar `the-crab-engram` como dependencia local.
- [ ] Integrar `notebooklm-rust-mcp` como dependencia local.
- [ ] Definir el trait `EventPublisher` para el bus de mensajes.
- [ ] Crear la estructura de carpetas inicial en el Vault de Obsidian.

## 📋 Hito 2: Sincronización con Obsidian
- [ ] Implementar parser de `Config/Materias.md` (formato YAML/Markdown).
- [ ] Crear el generador de `HOY.md` con lógica de prioridad básica.
- [ ] Crear el actualizador de `Progreso.md` basado en mock-data de maestría.
- [ ] Implementar un `Watcher` simple que detecte cambios en el vault.

## 📋 Hito 3: Tauri HUD (V1)
- [ ] Setup de Tauri con Rust (backend) y Framework ligero (frontend).
- [ ] Crear ventana flotante sin bordes para el Pomodoro.
- [ ] Implementar comunicación `Tauri -> Core` para iniciar/pausar sesiones.
- [ ] Mostrar la tarea actual extraída de `HOY.md` en el HUD.

## 📋 Hito 4: Memoria e Inteligencia
- [ ] Conectar el fin de sesión del Timer con `mem_save` de Engram.
- [ ] Implementar comando `test [topic]` que dispare el flujo de NotebookLM.
- [ ] Crear la UI de quiz en Tauri para responder preguntas sin salir del HUD.
- [ ] Automatizar la generación de `Repaso.md` usando `mem_reviews`.

## 📋 Hito 5: Refinamiento TDAH
- [ ] Implementar el "Modo Sobrecarga" (UI simplificada).
- [ ] Añadir lógica de "Andamiaje Socrático" en los prompts de IA para Matemáticas.
- [ ] Configurar alertas de anti-patrones (ej: sesión muy larga sin break).
- [ ] Generar resumen diario automático en `Sesiones/daily-summaries/`.

## 📋 Hito 6: Ciberdefensa Ops
- [ ] Crear plantillas específicas para toma de notas de Ciberdefensa.
- [ ] Integrar visualizador de grafos de conocimiento de Engram en el HUD.
- [ ] Pruebas finales de estrés con volumen real de PDFs de FADENA.
