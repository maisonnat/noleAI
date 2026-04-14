pub struct TemplateGenerator;

impl TemplateGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn cybersecurity_threat_model(&self) -> String {
        r#"# 🛡️ Modelo de Amenazas Ciberdefensa

## Información del Sistema
- **Nombre del Sistema**: (ej: Servidor Web Corporativo)
- **Propósito**: (ej: Hosting de aplicaciones internas)
- **Stakeholders**: (ej: IT, Usuarios Externos, Gerencia)

## Activos Identificados
| Activo | Tipo | Valor | Sensibilidad |
|--------|------|--------|-------------|
| (ej: Base de datos de clientes) | Datos | Alto | Confidencial |
| (ej: Servidor web) | Infraestructura | Medio | Público |
| | | | |

## Amenazas Potenciales
### 🔴 Alta Prioridad
- [ ] (ej: SQL Injection)
- [ ] (ej: Cross-Site Scripting (XSS))
- [ ] (ej: Denial of Service (DoS))

### 🟡 Media Prioridad
- [ ] (ej: Phishing a empleados)
- [ ] (ej: Malware interno)
- [ ] (ej: Fuga de datos)

### 🟢 Baja Prioridad
- [ ] (ej: Interrupciones de servicio no maliciosas)
- [ ] (ej: Errores de configuración)

## Controles Mitigación
| Amenaza | Control Implementado | Estado | Efectividad |
|----------|-------------------|---------|-------------|
| (ej: SQL Injection) | (ej: Prepared Statements) | (ej: Implementado) | (ej: Alta) |
| | | | |

## Residuales de Riesgo
- **Riesgo Total**: (ej: Medio)
- **Aceptación**: (ej: Sí, con monitoreo continuo)
- **Próxima Revisión**: (fecha)

## Notas Adicionales
> Espacio para observaciones, incidentes pasados, o cambios en el entorno

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    pub fn security_audit_log(&self) -> String {
        r#"# 📋 Log de Auditoría de Seguridad

## Información de la Auditoría
- **Fecha**: (auto-generado)
- **Auditor**: (nombre del auditor)
- **Tipo**: (ej: Rutinario, Incidente, Compliance)
- **Alcance**: (ej: Servidores, Red, Aplicaciones)

## Hallazgos
| ID | Severidad | Descripción | Componente Afectado | Estado |
|----|-----------|-------------|---------------------|--------|
| AUD-001 | (ej: Alta) | (descripción) | (componente) | (ej: Pendiente) |
| | | | | |

## Recomendaciones
### 🚨 Críticas (Inmediato)
1. (ej: Actualizar parches de seguridad críticos)
2. 

### ⚠️ Alta (24-48h)
1. (ej: Reforzar firewall de red)
2. 

### 📝 Media (1 semana)
1. (ej: Implementar autenticación MFA)
2. 

## Acciones Tomadas
| Hallazgo ID | Acción | Responsable | Fecha Implementación | Verificación |
|-------------|---------|-------------|---------------------|-------------|
| AUD-001 | (acción) | (nombre) | (fecha) | (estado) |

## Métricas de Seguridad
- **Vulnerabilidades Totales**: (número)
- **Críticas**: (número)
- **Resueltas**: (número)
- **Tiempo Promedio de Resolución**: (días)

## Conclusión
> Resumen del estado actual de seguridad y próximos pasos

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    pub fn incident_response_template(&self) -> String {
        r#"# 🚨 Plan de Respuesta a Incidentes

## 1. Detección y Notificación
- **Fecha/Hora de Detección**: (timestamp)
- **Detectado Por**: (nombre/sistema)
- **Tipo de Incidente**: (ej: Ransomware, Data Breach, DoS)
- **Severidad Inicial**: (1-5, donde 5 es crítica)

## 2. Evaluación Inicial
### Descripción del Incidente
- **Qué sucedió**: (descripción breve)
- **Qué sistemas afectados**: (lista)
- **Impacto estimado**: (ej: Crítico, Alto, Medio, Bajo)
- **Usuario afectados**: (número aproximado)

### Confirmación de Incidente
- [ ] Confirmado como incidente de seguridad
- [ ] Clasificado correctamente
- [ ] Comunicado al equipo de respuesta

## 3. Contención
### Acciones Inmediatas
- [ ] (ej: Aislar sistemas afectados)
- [ ] (ej: Bloquear IPs maliciosas)
- [ ] (ej: Cambiar credenciales comprometidas)
- [ ] (ej: Activar modo de emergencia)

### Tiempo de Contención
- **Inicio**: (timestamp)
- **Fin**: (timestamp)
- **Duración Total**: (minutos/horas)

## 4. Erradicación
### Pasos Realizados
1. (ej: Identificar causa raíz)
2. (ej: Eliminar malware)
3. (ej: Parchar vulnerabilidades)
4. (ej: Limpiar sistemas comprometidos)

### Verificación
- [ ] Sistemas limpios confirmados
- [ ] Causa raíz identificada
- [ ] Vulnerabilidades corregidas

## 5. Recuperación
### Plan de Restauración
- **Sistemas a restaurar**: (lista)
- **Backup utilizado**: (versión/fecha)
- **Prioridad de restauración**: (orden)

### Validación de Servicios
- [ ] Funcionalidad normal restaurada
- [ ] Verificación de integridad de datos
- [ ] Pruebas de funcionalidad
- [ ] Aprobación para operación normal

## 6. Lecciones Aprendidas
### Lo que funcionó bien
- 

### Lo que se puede mejorar
- 

### Cambios necesarios en procesos
- 

### Capacitación requerida
- 

## 7. Timeline Completo
| Tiempo | Acción | Responsable |
|---------|---------|-------------|
| 00:00 | Detección | (nombre) |
| 00:05 | Notificación | (nombre) |
| | | |

## 8. Documentación de Evidencia
- [ ] Capturas de pantalla
- [ ] Logs de sistema
- [ ] Análisis de malware
- [ ] Reporte de herramientas forenses

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    pub fn vulnerability_assessment(&self) -> String {
        r#"# 🔍 Evaluación de Vulnerabilidades

## Información General
- **Fecha**: (auto-generado)
- **Analista**: (nombre)
- **Objetivo**: (ej: Servidor Web Principal)
- **Metodología**: (ej: OWASP Top 10, CVE Scan, Manual)

## Resumen Ejecutivo
- **Vulnerabilidades Totales**: (número)
- **Críticas**: (número)
- **Altas**: (número)
- **Medias**: (número)
- **Bajas**: (número)

## Vulnerabilidades por Severidad

### 🔴 Críticas
| ID | Descripción | CVSS | Componente | Exploitabilidad |
|----|-------------|-------|-------------|----------------|
| CVE-XXXX-XXXX | (descripción) | (puntaje) | (componente) | (ej: Alta) |

### 🟠 Altas
| ID | Descripción | CVSS | Componente | Exploitabilidad |
|----|-------------|-------|-------------|----------------|
| | | | |

### 🟡 Medias
| ID | Descripción | CVSS | Componente | Exploitabilidad |
|----|-------------|-------|-------------|----------------|
| | | | |

### 🟢 Bajas
| ID | Descripción | CVSS | Componente | Exploitabilidad |
|----|-------------|-------|-------------|----------------|
| | | | |

## Análisis de Riesgo
- **Riesgo Total**: (ej: Alto)
- **Exposición**: (ej: Internet-facing)
- **Valor del Activo**: (ej: Alto)
- **Probabilidad de Explotación**: (ej: Alta)

## Recomendaciones de Remediación
### Prioridad Inmediata (1-3 días)
1. (ej: Parchear CVE-XXXX-XXXX)
2. 

### Prioridad Alta (1-2 semanas)
1. 
2. 

### Prioridad Media (1 mes)
1. 
2. 

## Plan de Validación
- **Método**: (ej: Re-scan, Testing manual)
- **Fecha de Revisión**: (fecha)
- **Responsable**: (nombre)

## Notas Adicionales
> Observaciones, contexto, o información relevante

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    pub fn compliance_checklist(&self, standard: &str) -> String {
        match standard {
            "GDPR" => self.gdpr_checklist(),
            "ISO27001" => self.iso27001_checklist(),
            "NIST" => self.nist_checklist(),
            _ => String::from("# Checklist de Compliance\n\nEstándar no reconocido. Por favor seleccione: GDPR, ISO27001, o NIST.\n"),
        }
    }

    fn gdpr_checklist(&self) -> String {
        r#"# ✅ Checklist GDPR (General Data Protection Regulation)

## Principios Fundamentales
- [ ] **Legalidad, equidad y transparencia**: Base legal legítima para el procesamiento
- [ ] **Limitación de propósito**: Solo datos necesarios para el objetivo
- [ ] **Minimización de datos**: Cantidad mínima de datos personales
- [ ] **Exactitud**: Datos actualizados y correctos
- [ ] **Limitación de almacenamiento**: No retener más tiempo del necesario
- [ ] **Integridad y confidencialidad**: Medidas de seguridad apropiadas
- [ ] **Responsabilidad**: Demostración de cumplimiento

## Derechos del Sujeto de Datos
- [ ] **Derecho a ser informado**: Política de privacidad clara
- [ ] **Derecho de acceso**: Sujetos pueden acceder a sus datos
- [ ] **Derecho de rectificación**: Corrección de datos inexactos
- [ ] **Derecho al olvido**: Eliminación de datos bajo ciertas condiciones
- [ ] **Derecho a la portabilidad**: Transferencia de datos a otro proveedor
- [ ] **Derecho a oponerse**: Oponerse al procesamiento automatizado

## Seguridad de Datos
- [ ] **Evaluación de impacto**: DPIA para procesamiento de alto riesgo
- [ ] **Encriptación**: Datos en tránsito y en reposo
- [ ] **Control de acceso**: Autenticación y autorización apropiadas
- [ ] **Backup y recuperación**: Procedimientos establecidos
- [ ] **Notificación de brechas**: Proceso para reportar incidentes en 72h

## Transferencias de Datos
- [ ] **Adequacy**: Países con protección adecuada de datos
- [ ] **SCCs**: Standard Contractual Clauses cuando sea necesario
- [ ] **BCR**: Binding Corporate Rules para transferencias internas

## Documentación y Gobernanza
- [ ] **Registro de actividades**: Log de procesamiento de datos
- [ ] **DPO**: Data Protection Officer designado (si aplica)
- [ ] **Política de privacidad**: Actualizada y accesible
- [ ] **Capacitación**: Personal entrenado en GDPR

## Verificación
- **Fecha de última revisión**: (fecha)
- **Próxima revisión programada**: (fecha)
- **Firmas de aprobación**: (responsables)

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    fn iso27001_checklist(&self) -> String {
        r#"# ✅ Checklist ISO/IEC 27001

## A.5 Políticas de Seguridad de la Información
- [ ] **A.5.1**: Dirección para seguridad de la información
- [ ] **A.5.2**: Políticas de seguridad de la información

## A.6 Organización de la Seguridad de la Información
- [ ] **A.6.1**: Organización interna
- [ ] **A.6.2**: Móviles y teletrabajo

## A.7 Seguridad de Recursos Humanos
- [ ] **A.7.1**: Antes del empleo
- [ ] **A.7.2**: Durante el empleo
- [ ] **A.7.3**: Terminación del empleo

## A.8 Gestión de Activos
- [ ] **A.8.1**: Responsabilidad sobre activos
- [ ] **A.8.2**: Clasificación de información
- [ ] **A.8.3**: Manejo de medios

## A.9 Control de Acceso
- [ ] **A.9.1**: Requisitos de control de acceso
- [ ] **A.9.2**: Gestión de acceso de usuarios
- [ ] **A.9.3**: Responsabilidades de usuarios
- [ ] **A.9.4**: Autenticación de sistemas
- [ ] **A.9.5**: Control de acceso a redes

## A.10 Criptografía
- [ ] **A.10.1**: Controles criptográficos

## A.11 Seguridad Física y Ambiental
- [ ] **A.11.1**: Áreas seguras
- [ ] **A.11.2**: Equipos

## A.12 Seguridad de Operaciones
- [ ] **A.12.1**: Procedimientos operativos
- [ ] **A.12.2**: Protección contra malware
- [ ] **A.12.3**: Respaldo
- [ ] **A.12.4**: Logging y monitoreo
- [ ] **A.12.5**: Control de software operativo
- [ ] **A.12.6**: Gestión de vulnerabilidades técnicas
- [ ] **A.12.7**: Consideraciones de auditoría

## A.13 Seguridad de Comunicaciones
- [ ] **A.13.1**: Seguridad de gestión de red
- [ ] **A.13.2**: Transferencia de información

## A.14 Adquisición, Desarrollo y Mantenimiento
- [ ] **A.14.1**: Requisitos de seguridad
- [ ] **A.14.2**: Seguridad en desarrollo
- [ ] **A.14.3**: Datos de prueba

## A.15 Relaciones con Proveedores
- [ ] **A.15.1**: Seguridad en relaciones con proveedores
- [ ] **A.15.2**: Gestión de suministro de servicios

## A.16 Gestión de Incidentes de Seguridad de la Información
- [ ] **A.16.1**: Gestión de incidentes
- [ ] **A.16.2**: Mejora en gestión de incidentes

## A.17 Continuidad de Negocios
- [ ] **A.17.1**: Continuidad de seguridad de la información

## A.18 Cumplimiento
- [ ] **A.18.1**: Cumplimiento legal
- [ ] **A.18.2**: Revisiones de políticas
- [ ] **A.18.3**: Cumplimiento técnico

## Evaluación
- **Fecha de auditoría**: (fecha)
- **Resultado**: (ej: Conforme, No conforme)
- **Observaciones**: (notas)
- **Acciones correctivas**: (lista)

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }

    fn nist_checklist(&self) -> String {
        r#"# ✅ Checklist NIST Cybersecurity Framework (CSF)

## 1. Identificar (IDENTIFY)
- [ ] **ID.AM**: Gestión de Activos
  - [ ] Hardware
  - [ ] Software
  - [ ] Sistemas
- [ ] **ID.BE**: Gobierno del Comportamiento Empresarial
  - [ ] Políticas de seguridad
  - [ ] Procedimientos
- [ ] **ID.GV**: Contexto de Gobernanza
  - [ ] Roles y responsabilidades
  - [ ] Alineamiento legal
- [ ] **ID.RA**: Evaluación de Riesgo
  - [ ] Análisis de riesgos
  - [ ] Evaluación de impacto
- [ ] **ID.RM**: Gestión de Riesgo
  - [ ] Estrategia de respuesta
  - [ ] Monitoreo continuo

## 2. Proteger (PROTECT)
- [ ] **PR.AC**: Control de Acceso
  - [ ] Identidad y autenticación
  - [ ] Autorización
- [ ] **PR.AT**: Conciencia y Capacitación
  - [ ] Entrenamiento de personal
  - [ ] Phishing awareness
- [ ] **PR.DS**: Seguridad de Datos
  - [ ] Protección en reposo
  - [ ] Protección en tránsito
- [ ] **PR.IP**: Protección de Infraestructura
  - [ ] Firewalls
  - [ ] IDS/IPS
- [ ] **PR.PS**: Tecnologías de Protección
  - [ ] Encriptación
  - [ ] MFA
- [ ] **PR.MA**: Mantenimiento
  - [ ] Parches
  - [ ] Actualizaciones

## 3. Detectar (DETECT)
- [ ] **DE.CM**: Comportamiento Anómalo
  - [ ] Análisis de comportamiento
  - [ ] Detección de anomalías
- [ ] **DE.AE**: Eventos de Seguridad
  - [ ] Colección de logs
  - [ ] Monitoreo en tiempo real
- [ ] **DE.DP**: Actividades de Detección
  - [ ] Alertas tempranas
  - [ ] Análisis forense

## 4. Responder (RESPOND)
- [ ] **RS.RP**: Planificación de Respuesta
  - [ ] Plan de respuesta documentado
  - [ ] Simulacros regulares
- [ ] **RS.CO**: Comunicaciones
  - [ ] Procedimientos de notificación
  - [ ] Comunicación con stakeholders
- [ ] **RS.AN**: Análisis
  - [ ] Investigación de incidentes
  - [ ] Análisis de causa raíz
- [ ] **RS.MI**: Mitigación
  - [ ] Contención inmediata
  - [ ] Erradicación
- [ ] **RS.IM**: Mejoras
  - [ ] Lecciones aprendidas
  - [ ] Mejora continua

## 5. Recuperar (RECOVER)
- [ ] **RC.RP**: Planificación de Recuperación
  - [ ] Plan de DRP documentado
  - [ ] Pruebas de recuperación
- [ ] **RC.CO**: Comunicaciones de Recuperación
  - [ ] Comunicación durante incidente
  - [ ] Post-incidente
- [ ] **RC.IM**: Mejoras de Recuperación
  - [ ] Actualización de planes
  - [ ] Reforzamiento de controles

## Evaluación de Madurez
- **Nivel Actual**: (1-5, donde 5 es óptimo)
- **Objetivo**: (nivel deseado)
- **Brecha**: (diferencia y plan para cerrarla)

## Métricas Clave
- **Incidentes totales**: (número)
- **Tiempo de detección**: (promedio)
- **Tiempo de respuesta**: (promedio)
- **Tiempo de recuperación**: (promedio)

---
*Plantilla generada por NoleAI - Módulo de Ciberdefensa*
"#
        .to_string()
    }
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_model_template() {
        let generator = TemplateGenerator::new();
        let template = generator.cybersecurity_threat_model();

        assert!(template.contains("Modelo de Amenazas"));
        assert!(template.contains("Activos Identificados"));
        assert!(template.contains("Amenazas Potenciales"));
    }

    #[test]
    fn test_security_audit_template() {
        let generator = TemplateGenerator::new();
        let template = generator.security_audit_log();

        assert!(template.contains("Log de Auditoría"));
        assert!(template.contains("Hallazgos"));
        assert!(template.contains("Recomendaciones"));
    }

    #[test]
    fn test_incident_response_template() {
        let generator = TemplateGenerator::new();
        let template = generator.incident_response_template();

        assert!(template.contains("Plan de Respuesta a Incidentes"));
        assert!(template.contains("Detección y Notificación"));
        assert!(template.contains("Contención"));
    }

    #[test]
    fn test_vulnerability_assessment_template() {
        let generator = TemplateGenerator::new();
        let template = generator.vulnerability_assessment();

        assert!(template.contains("Evaluación de Vulnerabilidades"));
        assert!(template.contains("Críticas"));
        assert!(template.contains("Recomendaciones de Remediación"));
    }

    #[test]
    fn test_compliance_checklists() {
        let generator = TemplateGenerator::new();

        let gdpr = generator.compliance_checklist("GDPR");
        assert!(gdpr.contains("GDPR"));
        assert!(gdpr.contains("Principios Fundamentales"));

        let iso = generator.compliance_checklist("ISO27001");
        assert!(iso.contains("ISO/IEC 27001"));

        let nist = generator.compliance_checklist("NIST");
        assert!(nist.contains("NIST Cybersecurity Framework"));
    }
}
