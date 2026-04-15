# NoleAI: Sistema de Función Ejecutiva Externa (Especificación)

**Versión:** 1.0  
**Arquitectura:** Modular Rust Workspace + Tauri HUD + Obsidian Battlefield  
**Estado:** Investigación y Diseño (Inquiry)

## 1. Visión y Propósito
NoleAI es un ecosistema diseñado para actuar como una corteza prefrontal externa. Su objetivo es reducir la carga cognitiva de un estudiante universitario con TDAH (Enfoque: Ciberdefensa / Matemáticas), externalizando la planificación, priorización y memoria de trabajo.

## 2. Componentes del Ecosistema

### A. Orquestador (Rust Workspace)
El "cerebro" central que coordina los módulos existentes y nuevos:
*   **the-crab-engram (🦀):** Motor de memoria persistente. Almacena sesiones, niveles de maestría y detecta anti-patrones de conducta.
*   **notebooklm-rust-mcp:** Bridge para la ingesta de material. Genera quizzes y resúmenes desde PDFs de FADENA.
*   **nole-core:** Lógica de orquestación, bus de eventos y algoritmos de TDAH (Priorización, FSRS).

### B. Interfaz (Tauri HUD)
Un panel de instrumentos (HUD) minimalista y ligero:
*   **Timer Pomodoro:** Vinculado al registro de sesiones en el Engram.
*   **Visualización de Tarea:** Muestra solo la tarea actual para evitar el overwhelm.
*   **Modo "No Puedo":** Botón de emergencia para estados de sobrecarga sensorial o cognitiva.

### C. Almacenamiento (Obsidian Vault)
El repositorio de conocimiento en archivos Markdown locales:
*   `HOY.md`: Plan diario generado por el Core.
*   `Progreso.md`: Dashboard de maestría actualizado por el Engram.
*   `Repaso.md`: Lista de repetición espaciada (FSRS).
*   `Config/`: Definiciones de materias y perfiles de energía TDAH.

## 3. Principios de Diseño para TDAH
1.  **Fricción Cero:** El sistema debe "succionar" PDFs y "escribir" planes sin intervención manual constante.
2.  **Decisión Centralizada:** El sistema decide qué estudiar; el alumno ejecuta.
3.  **Andamiaje Socrático:** Especialmente en Matemáticas, el sistema guía procesos en lugar de dar soluciones.
4.  **Resiliencia Emocional:** El sistema no usa lenguaje de culpa por días perdidos; realiza "soft resets" automáticos.

## 4. Integraciones Técnicas
*   **MCP (Model Context Protocol):** Para comunicación fluida con NotebookLM.
*   **Local-First:** Todo el procesamiento y los datos residen en la máquina local por privacidad y velocidad.
*   **LaTeX Support:** Renderizado nativo de fórmulas matemáticas en el HUD y Obsidian.
