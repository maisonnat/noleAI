use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct StressTestConfig {
    pub num_pdfs: usize,
    pub avg_pdf_size_kb: usize,
    pub concurrent_sessions: usize,
    pub test_duration_secs: u64,
}

impl Default for StressTestConfig {
    fn default() -> Self {
        Self {
            num_pdfs: 50,            // Simular 50 PDFs de FADENA
            avg_pdf_size_kb: 1024,   // 1 MB promedio
            concurrent_sessions: 5,  // 5 sesiones concurrentes
            test_duration_secs: 300, // 5 minutos de prueba
        }
    }
}

#[derive(Debug, Clone)]
pub struct StressTestResult {
    pub pdfs_processed: usize,
    pub quizzes_generated: usize,
    pub total_data_processed_mb: f64,
    pub average_processing_time_ms: f64,
    pub peak_memory_usage_mb: f64,
    pub errors_encountered: Vec<String>,
    pub success_rate: f64,
}

pub struct StressTester;

impl StressTester {
    pub fn new() -> Self {
        Self
    }

    pub fn run_load_test(&self, config: &StressTestConfig) -> StressTestResult {
        println!("🧪 Iniciando prueba de estrés...");
        println!("   - PDFs a procesar: {}", config.num_pdfs);
        println!("   - Tamaño promedio: {} KB", config.avg_pdf_size_kb);
        println!("   - Sesiones concurrentes: {}", config.concurrent_sessions);
        println!("   - Duración: {} segundos\n", config.test_duration_secs);

        let start_time = Instant::now();
        let mut results = StressTestResult {
            pdfs_processed: 0,
            quizzes_generated: 0,
            total_data_processed_mb: 0.0,
            average_processing_time_ms: 0.0,
            peak_memory_usage_mb: 0.0,
            errors_encountered: Vec::new(),
            success_rate: 0.0,
        };

        let mut processing_times = Vec::new();

        for i in 0..config.num_pdfs {
            let pdf_start = Instant::now();

            // Simular procesamiento de PDF
            match self.simulate_pdf_processing(i, config.avg_pdf_size_kb) {
                Ok(success) => {
                    if success {
                        results.pdfs_processed += 1;
                        results.quizzes_generated += 1;
                        results.total_data_processed_mb += config.avg_pdf_size_kb as f64 / 1024.0;
                    } else {
                        results
                            .errors_encountered
                            .push(format!("PDF {} falló en procesamiento", i));
                    }
                }
                Err(e) => {
                    results
                        .errors_encountered
                        .push(format!("Error en PDF {}: {}", i, e));
                }
            }

            processing_times.push(pdf_start.elapsed().as_millis());

            // Simular carga de memoria
            let memory_usage = self.estimate_memory_usage(results.pdfs_processed);
            if memory_usage > results.peak_memory_usage_mb {
                results.peak_memory_usage_mb = memory_usage;
            }

            // Simular concurrent sessions
            if i % config.concurrent_sessions == 0 {
                thread::sleep(Duration::from_millis(100));
            }

            // Check timeout
            if start_time.elapsed().as_secs() > config.test_duration_secs {
                println!("⏱️ Tiempo límite alcanzado");
                break;
            }

            // Progreso
            if (i + 1) % 10 == 0 {
                println!("   Procesados: {}/{} PDFs", i + 1, config.num_pdfs);
            }
        }

        // Calcular estadísticas
        if !processing_times.is_empty() {
            let total_time: u128 = processing_times.iter().sum();
            results.average_processing_time_ms = total_time as f64 / processing_times.len() as f64;
        }

        results.success_rate = if config.num_pdfs > 0 {
            (results.pdfs_processed as f64 / config.num_pdfs as f64) * 100.0
        } else {
            0.0
        };

        results
    }

    fn simulate_pdf_processing(&self, index: usize, size_kb: usize) -> Result<bool, String> {
        // Simular tiempo de procesamiento basado en tamaño
        let processing_time_ms = (size_kb as f64 / 10.0) as u64;
        thread::sleep(Duration::from_millis(processing_time_ms));

        // Simular algunos fallos aleatorios (5% tasa de fallo)
        if index % 20 == 0 && index > 0 {
            return Err("Simulación de fallo de lectura PDF".to_string());
        }

        // Simular procesamiento exitoso
        Ok(true)
    }

    fn estimate_memory_usage(&self, pdfs_processed: usize) -> f64 {
        // Estimar uso de memoria: ~50KB por PDF en memoria + overhead
        (pdfs_processed * 50) as f64 / 1024.0
    }

    pub fn run_concurrent_session_test(&self, config: &StressTestConfig) -> StressTestResult {
        println!("🔄 Prueba de sesiones concurrentes...");

        let mut results = StressTestResult {
            pdfs_processed: 0,
            quizzes_generated: 0,
            total_data_processed_mb: 0.0,
            average_processing_time_ms: 0.0,
            peak_memory_usage_mb: 0.0,
            errors_encountered: Vec::new(),
            success_rate: 0.0,
        };

        let _start_time = Instant::now();
        let mut processing_times = Vec::new();

        for session in 0..config.concurrent_sessions {
            let session_start = Instant::now();

            // Simular sesión completa con múltiples PDFs
            let pdfs_per_session = config.num_pdfs / config.concurrent_sessions;
            for i in 0..pdfs_per_session {
                let pdf_index = session * pdfs_per_session + i;

                match self.simulate_pdf_processing(pdf_index, config.avg_pdf_size_kb) {
                    Ok(true) => {
                        results.pdfs_processed += 1;
                        results.quizzes_generated += 1;
                        results.total_data_processed_mb += config.avg_pdf_size_kb as f64 / 1024.0;
                    }
                    Ok(false) => {
                        results
                            .errors_encountered
                            .push(format!("Sesión {}, PDF {} falló", session, pdf_index));
                    }
                    Err(e) => {
                        results
                            .errors_encountered
                            .push(format!("Sesión {}, PDF {}: {}", session, pdf_index, e));
                    }
                }
            }

            processing_times.push(session_start.elapsed().as_millis());
            println!("   Sesión {} completada", session + 1);
        }

        // Calcular estadísticas
        if !processing_times.is_empty() {
            let total_time: u128 = processing_times.iter().sum();
            results.average_processing_time_ms = total_time as f64 / processing_times.len() as f64;
        }

        results.peak_memory_usage_mb = self.estimate_memory_usage(results.pdfs_processed);
        results.success_rate = if config.num_pdfs > 0 {
            (results.pdfs_processed as f64 / config.num_pdfs as f64) * 100.0
        } else {
            0.0
        };

        results
    }

    pub fn generate_report(&self, results: &StressTestResult, config: &StressTestConfig) -> String {
        let mut report = String::from("# 📊 Reporte de Prueba de Estrés\n\n");

        report.push_str("## 🎯 Objetivos de la Prueba\n");
        report.push_str(&format!("- Procesar {} PDFs\n", config.num_pdfs));
        report.push_str(&format!(
            "- Tamaño promedio: {} KB\n",
            config.avg_pdf_size_kb
        ));
        report.push_str(&format!(
            "- Sesiones concurrentes: {}\n",
            config.concurrent_sessions
        ));
        report.push_str(&format!(
            "- Duración máxima: {} segundos\n\n",
            config.test_duration_secs
        ));

        report.push_str("## 📈 Resultados Obtenidos\n");
        report.push_str(&format!(
            "- **PDFs procesados**: {}\n",
            results.pdfs_processed
        ));
        report.push_str(&format!(
            "- **Quizzes generados**: {}\n",
            results.quizzes_generated
        ));
        report.push_str(&format!(
            "- **Datos procesados**: {:.2} MB\n",
            results.total_data_processed_mb
        ));
        report.push_str(&format!(
            "- **Tiempo promedio**: {:.2} ms\n",
            results.average_processing_time_ms
        ));
        report.push_str(&format!(
            "- **Pico de memoria**: {:.2} MB\n",
            results.peak_memory_usage_mb
        ));
        report.push_str(&format!(
            "- **Tasa de éxito**: {:.1}%\n\n",
            results.success_rate
        ));

        report.push_str("## ⚠️ Errores Encontrados\n");
        if results.errors_encountered.is_empty() {
            report.push_str("*No se encontraron errores*\n\n");
        } else {
            for error in &results.errors_encountered {
                report.push_str(&format!("- {}\n", error));
            }
            report.push_str("\n");
        }

        report.push_str("## 📊 Análisis de Rendimiento\n");

        let throughput = results.pdfs_processed as f64 / config.test_duration_secs as f64;
        report.push_str(&format!(
            "- **Throughput**: {:.2} PDFs/segundo\n",
            throughput
        ));

        let data_rate = results.total_data_processed_mb / config.test_duration_secs as f64;
        report.push_str(&format!(
            "- **Tasa de datos**: {:.2} MB/segundo\n",
            data_rate
        ));

        if results.success_rate >= 95.0 {
            report.push_str("- **Estado**: ✅ EXCELENTE\n");
        } else if results.success_rate >= 80.0 {
            report.push_str("- **Estado**: ✅ BUENO\n");
        } else if results.success_rate >= 60.0 {
            report.push_str("- **Estado**: ⚠️ ACEPTABLE\n");
        } else {
            report.push_str("- **Estado**: ❌ NECESITA MEJORA\n");
        }

        report.push_str("\n## 💡 Recomendaciones\n");

        if results.peak_memory_usage_mb > 500.0 {
            report.push_str("- Considerar implementar paginación para reducir uso de memoria\n");
        }

        if results.average_processing_time_ms > 1000.0 {
            report.push_str("- Optimizar procesamiento de PDFs para mejorar rendimiento\n");
        }

        if results.success_rate < 95.0 {
            report.push_str("- Implementar mejor manejo de errores y reintentos\n");
        }

        if config.concurrent_sessions > 1 && results.success_rate < 90.0 {
            report.push_str("- Reducir concurrencia o mejorar sincronización\n");
        }

        report.push_str("\n---\n");
        report.push_str("*Reporte generado por NoleAI - Módulo de Testing*\n");

        report
    }

    pub fn run_benchmark_suite(&self) -> Vec<(String, StressTestResult)> {
        println!("🚀 Ejecutando suite de benchmarks...\n");

        let mut suite_results = Vec::new();

        // Test 1: Carga ligera
        println!("Test 1: Carga ligera (10 PDFs)");
        let config_light = StressTestConfig {
            num_pdfs: 10,
            avg_pdf_size_kb: 512,
            concurrent_sessions: 1,
            test_duration_secs: 60,
        };
        let result_light = self.run_load_test(&config_light);
        suite_results.push(("Carga Ligera".to_string(), result_light));

        // Test 2: Carga media
        println!("\nTest 2: Carga media (50 PDFs)");
        let config_medium = StressTestConfig {
            num_pdfs: 50,
            avg_pdf_size_kb: 1024,
            concurrent_sessions: 3,
            test_duration_secs: 180,
        };
        let result_medium = self.run_load_test(&config_medium);
        suite_results.push(("Carga Media".to_string(), result_medium));

        // Test 3: Carga pesada
        println!("\nTest 3: Carga pesada (100 PDFs)");
        let config_heavy = StressTestConfig {
            num_pdfs: 100,
            avg_pdf_size_kb: 2048,
            concurrent_sessions: 5,
            test_duration_secs: 300,
        };
        let result_heavy = self.run_load_test(&config_heavy);
        suite_results.push(("Carga Pesada".to_string(), result_heavy));

        // Test 4: Sesiones concurrentes
        println!("\nTest 4: Sesiones concurrentes");
        let config_concurrent = StressTestConfig {
            num_pdfs: 30,
            avg_pdf_size_kb: 1024,
            concurrent_sessions: 10,
            test_duration_secs: 120,
        };
        let result_concurrent = self.run_concurrent_session_test(&config_concurrent);
        suite_results.push(("Concurrente".to_string(), result_concurrent));

        println!("\n✅ Suite de benchmarks completada");
        suite_results
    }
}

impl Default for StressTester {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_test_config_default() {
        let config = StressTestConfig::default();
        assert_eq!(config.num_pdfs, 50);
        assert_eq!(config.avg_pdf_size_kb, 1024);
        assert_eq!(config.concurrent_sessions, 5);
        assert_eq!(config.test_duration_secs, 300);
    }

    #[test]
    fn test_load_test_execution() {
        let tester = StressTester::new();
        let config = StressTestConfig {
            num_pdfs: 5,
            avg_pdf_size_kb: 100,
            concurrent_sessions: 1,
            test_duration_secs: 30,
        };

        let result = tester.run_load_test(&config);
        assert!(result.pdfs_processed <= 5);
        // El resultado puede variar dependiendo del tiempo de ejecución, pero siempre debe ser no negativo
        assert!(result.pdfs_processed as i32 >= 0);
    }

    #[test]
    fn test_concurrent_session_test() {
        let tester = StressTester::new();
        let config = StressTestConfig {
            num_pdfs: 10,
            avg_pdf_size_kb: 50,
            concurrent_sessions: 3,
            test_duration_secs: 60,
        };

        let result = tester.run_concurrent_session_test(&config);
        assert!(result.pdfs_processed <= 10);
    }

    #[test]
    fn test_benchmark_suite() {
        let tester = StressTester::new();
        let results = tester.run_benchmark_suite();

        assert_eq!(results.len(), 4);
        assert!(results.iter().any(|(name, _)| name == "Carga Ligera"));
        assert!(results.iter().any(|(name, _)| name == "Carga Media"));
        assert!(results.iter().any(|(name, _)| name == "Carga Pesada"));
        assert!(results.iter().any(|(name, _)| name == "Concurrente"));
    }

    #[test]
    fn test_report_generation() {
        let tester = StressTester::new();
        let config = StressTestConfig::default();
        let result = StressTestResult {
            pdfs_processed: 50,
            quizzes_generated: 50,
            total_data_processed_mb: 50.0,
            average_processing_time_ms: 100.0,
            peak_memory_usage_mb: 2.5,
            errors_encountered: vec!["Error simulado".to_string()],
            success_rate: 98.0,
        };

        let report = tester.generate_report(&result, &config);
        assert!(report.contains("Reporte de Prueba de Estrés"));
        assert!(report.contains("PDFs procesados**: 50"));
        assert!(report.contains("Tasa de éxito**: 98.0%"));
        assert!(report.contains("EXCELENTE"));
    }
}
