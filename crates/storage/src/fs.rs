use std::path::{Path, PathBuf};

use crate::store::{Result, Store, StoreError};

/// Filesystem-backed [`Store`] — the default backend (Principle III). Keys are mapped to
/// paths under a base directory; the base is resolved by the host (e.g. the Tauri shell's
/// app-data dir) so this crate stays path-agnostic.
#[derive(Debug, Clone)]
pub struct FsStore {
    base: PathBuf,
}

impl FsStore {
    pub fn new(base: impl Into<PathBuf>) -> Self {
        FsStore { base: base.into() }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        // Keys are forward-slash separated; reject traversal.
        let mut p = self.base.clone();
        for seg in key
            .split('/')
            .filter(|s| !s.is_empty() && *s != "." && *s != "..")
        {
            p.push(seg);
        }
        p
    }

    fn collect(dir: &Path, base: &Path, out: &mut Vec<String>) -> std::io::Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::collect(&path, base, out)?;
            } else if let Ok(rel) = path.strip_prefix(base) {
                out.push(rel.to_string_lossy().replace('\\', "/"));
            }
        }
        Ok(())
    }
}

impl Store for FsStore {
    fn get(&self, key: &str) -> Result<Vec<u8>> {
        let path = self.path_for(key);
        match std::fs::read(&path) {
            Ok(b) => Ok(b),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                Err(StoreError::NotFound(key.to_string()))
            }
            Err(e) => Err(StoreError::Io(e.to_string())),
        }
    }

    fn put(&self, key: &str, bytes: &[u8]) -> Result<()> {
        let path = self.path_for(key);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| StoreError::Io(e.to_string()))?;
        }
        std::fs::write(&path, bytes).map_err(|e| StoreError::Io(e.to_string()))
    }

    fn exists(&self, key: &str) -> bool {
        self.path_for(key).exists()
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let dir = self.path_for(prefix);
        Self::collect(&dir, &self.base, &mut out).map_err(|e| StoreError::Io(e.to_string()))?;
        out.sort();
        Ok(out)
    }

    fn delete_prefix(&self, prefix: &str) -> Result<()> {
        let dir = self.path_for(prefix);
        if dir.is_dir() {
            std::fs::remove_dir_all(&dir).map_err(|e| StoreError::Io(e.to_string()))?;
        } else if dir.exists() {
            std::fs::remove_file(&dir).map_err(|e| StoreError::Io(e.to_string()))?;
        }
        Ok(())
    }
}
