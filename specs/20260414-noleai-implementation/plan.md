# NoleAI: Plan de Ejecución Modular

Este plan detalla la construcción de NoleAI utilizando una estrategia de "funcionalidad incremental" para evitar el abandono del proyecto y asegurar que cada paso ayude al estudio actual.

## Fase 1: Cimientos y Orquestación (Semanas 1-2)
**Objetivo:** Establecer la comunicación entre los componentes existentes.
*   Configurar el `Cargo workspace` unificando `the-crab-engram` y `notebooklm-rust-mcp`.
*   Implementar el `Event Bus` central en Rust para coordinar señales entre memoria y UI.
*   Desarrollar el `nole-obsidian-sync`: lector de `Config/Materias.md` y escritor de `HOY.md`.

## Fase 2: El HUD TDAH (Semanas 3-4)
**Objetivo:** Crear la interfaz que reduce el ruido mental.
*   Inicializar el proyecto `nole-hud` con Tauri.
*   Vincular el Timer del HUD con el guardado automático de sesiones en `the-crab-engram`.
*   Implementar notificaciones proactivas basadas en el tiempo de sesión (Anti-hiperfoco).

## Fase 3: Inteligencia e Ingesta (Semanas 5-6)
**Objetivo:** Automatizar la creación de material de estudio.
*   Establecer el flujo: PDF en carpeta → `notebooklm-rust-mcp` → Quiz en el HUD.
*   Integrar el algoritmo de Repetición Espaciada (FSRS) de Engram para actualizar `Repaso.md`.
*   Crear el dashboard de maestría visual en el HUD extrayendo datos del Engram.

## Fase 4: Especialización y Refinamiento (Semanas 7-8)
**Objetivo:** Adaptar el sistema a materias complejas y conducta TDAH.
*   Activar el botón "No Puedo" (Simplificación de interfaz y plan).
*   Implementar el Tutor Socrático para procesos de Álgebra y Análisis Matemático.
*   Activar el detector de anti-patrones para sugerir descansos inteligentes.

## Estrategia de Validación
*   **Manual:** El alumno verifica que el plan diario sea realista.
*   **Empírica:** Comparar las horas de estudio "Deep Work" registradas antes y después de usar el sistema.
*   **Técnica:** Tests de integridad de datos en el Engram y validación de esquemas JSON para MCP.
