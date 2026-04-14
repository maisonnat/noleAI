use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;

/// Inicia un watcher síncrono en un hilo aparte para detectar cambios en el Vault.
pub fn start_watcher(vault_path: &Path) -> notify::Result<RecommendedWatcher> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(vault_path, RecursiveMode::Recursive)?;

    println!("👀 Watcher iniciado para: {:?}", vault_path);

    // Hilo dedicado para escuchar eventos
    std::thread::spawn(move || {
        while let Ok(res) = rx.recv() {
            match res {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                        for path in event.paths {
                            // Aquí se emitirá un Event::ObsidianVaultChanged al EventBus en el futuro
                            println!("🔄 Cambio detectado en: {:?}", path);
                        }
                    }
                }
                Err(e) => eprintln!("⚠️ Error en watcher: {:?}", e),
            }
        }
    });

    Ok(watcher)
}
