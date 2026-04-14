pub struct SocraticTutor;

impl SocraticTutor {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_socratic_prompt(&self, topic: &str, problem: &str) -> String {
        let mut prompt = String::new();

        prompt.push_str("## Andamiaje Socrático para Matemáticas\n\n");
        prompt.push_str(&format!("**Tema**: {}\n", topic));
        prompt.push_str(&format!("**Problema**: {}\n\n", problem));

        prompt.push_str("### Guía de Pensamiento (No leas esto al principio):\n");
        prompt.push_str("1. **Comprensión**: ¿Qué entiendes del problema? ¿Qué te pide?\n");
        prompt
            .push_str("2. **Identificación**: ¿Qué conceptos o fórmulas podrían ser relevantes?\n");
        prompt.push_str("3. **Planificación**: ¿Cómo podrías abordar el problema paso a paso?\n");
        prompt.push_str("4. **Ejecución**: Intenta resolverlo con el plan que hiciste.\n");
        prompt.push_str("5. **Verificación**: ¿Tu respuesta tiene sentido? ¿Puedes verificarla de otra forma?\n\n");

        prompt.push_str("### Preguntas Guía (Haz estas preguntas si se bloquea):\n");
        prompt.push_str("- ¿Qué información te da el problema?\n");
        prompt.push_str("- ¿Qué estás buscando exactamente?\n");
        prompt.push_str("- ¿Has visto algo similar antes?\n");
        prompt.push_str("- ¿Puedes dibujar un diagrama?\n");
        prompt.push_str("- ¿Hay restricciones o condiciones especiales?\n");
        prompt.push_str("- ¿Puedes simplificar el problema?\n\n");

        prompt.push_str("### Estrategias Adicionales:\n");
        prompt.push_str(
            "- **Trabajar hacia atrás**: Empieza desde la respuesta y ve qué necesitas.\n",
        );
        prompt.push_str("- **Casos especiales**: ¿Qué pasa si x = 0, 1, o ∞?\n");
        prompt.push_str("- **Patrones**: ¿Ves algún patrón o regularidad?\n");
        prompt.push_str(
            "- **Descomposición**: ¿Puedes dividir el problema en partes más pequeñas?\n\n",
        );

        prompt.push_str("> **Nota**: Este andamiaje está diseñado para desarrollar pensamiento crítico y autodirección, no para dar respuestas directas.\n");

        prompt
    }

    pub fn generate_hint_sequence(&self, problem: &str) -> Vec<String> {
        vec![
            format!(
                "Paso 1 para '{}': Lee el problema cuidadosamente y subraya la información clave.",
                problem
            ),
            format!(
                "Paso 2 para '{}': ¿Qué fórmulas o conceptos crees que se aplican aquí?",
                problem
            ),
            format!(
                "Paso 3 para '{}': Intenta escribir los primeros pasos de tu solución.",
                problem
            ),
            format!(
                "Paso 4 para '{}': Si estás atascado, intenta dibujar un diagrama.",
                problem
            ),
            format!(
                "Paso 5 para '{}': Verifica tu respuesta razonando si tiene sentido.",
                problem
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_socratic_prompt() {
        let tutor = SocraticTutor::new();
        let prompt = tutor.generate_socratic_prompt("Álgebra", "Resolver 2x + 5 = 15");

        assert!(prompt.contains("Andamiaje Socrático"));
        assert!(prompt.contains("Álgebra"));
        assert!(prompt.contains("Resolver 2x + 5 = 15"));
        assert!(prompt.contains("Preguntas Guía"));
    }

    #[test]
    fn test_generate_hint_sequence() {
        let tutor = SocraticTutor::new();
        let hints = tutor.generate_hint_sequence("Derivadas");

        assert_eq!(hints.len(), 5);
        assert!(hints[0].contains("Paso 1"));
        assert!(hints[4].contains("Paso 5"));
    }
}
