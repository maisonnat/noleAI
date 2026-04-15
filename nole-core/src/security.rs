use std::path::{Path, PathBuf};

/// Validate that a path does not escape its intended parent directory.
///
/// Prevents path traversal attacks by resolving the path and checking
/// it starts with the expected prefix.
pub fn validate_path_within(base: &Path, target: &Path) -> Result<PathBuf, String> {
    let canonical_base = base
        .canonicalize()
        .map_err(|e| format!("Cannot canonicalize base path: {}", e))?;

    let resolved = if target.exists() {
        target
            .canonicalize()
            .map_err(|e| format!("Cannot canonicalize target path: {}", e))?
    } else {
        let resolved = canonical_base.join(target);
        resolved
    };

    if !resolved.starts_with(&canonical_base) {
        return Err(format!(
            "Path traversal detected: {} escapes {}",
            resolved.display(),
            canonical_base.display()
        ));
    }

    Ok(resolved)
}

/// Ensure a directory is created with restrictive permissions (owner-only on Unix).
///
/// On Unix, sets mode 0o700 (rwx------). On other platforms, relies on default umask.
pub fn create_secure_dir(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(path, perms)?;
        }
    }
    Ok(())
}

/// Validate that a data storage path is safe to use.
///
/// Checks for path traversal, ensures parent exists, and creates
/// the directory with secure permissions.
pub fn ensure_secure_data_dir(base: &Path) -> Result<PathBuf, String> {
    let base = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());

    if !base.is_absolute() {
        return Err("Data directory path must be absolute".to_string());
    }

    let components = base.components().collect::<Vec<_>>();
    for comp in &components {
        if let std::path::Component::ParentDir = comp {
            return Err("Parent directory components not allowed in data path".to_string());
        }
    }

    create_secure_dir(&base).map_err(|e| format!("Failed to create secure dir: {}", e))?;
    Ok(base)
}
