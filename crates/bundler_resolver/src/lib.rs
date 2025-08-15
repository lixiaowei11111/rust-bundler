//! Module resolver for the Rust bundler
//!
//! This crate handles resolving module imports to actual file paths,
//! similar to how Node.js and Webpack resolve modules.

use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResolverError {
    #[error("Module not found: {request} from {context}")]
    ModuleNotFound { request: String, context: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ResolverError>;

pub struct Resolver {
    extensions: Vec<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            extensions: vec![".js".to_string(), ".ts".to_string(), ".json".to_string()],
        }
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    pub async fn resolve(&self, request: &str, context: &Path) -> Result<PathBuf> {
        // Handle relative imports
        if request.starts_with("./") || request.starts_with("../") {
            return self.resolve_relative(request, context).await;
        }

        // Handle absolute imports (for now, treat as relative to context)
        // In a real implementation, you'd check node_modules, etc.
        self.resolve_relative(request, context).await
    }

    async fn resolve_relative(&self, request: &str, context: &Path) -> Result<PathBuf> {
        let context_dir = if context.is_file() {
            context.parent().unwrap_or(context)
        } else {
            context
        };

        let mut candidate = context_dir.join(request);

        // Try the exact path first
        if candidate.exists() {
            return Ok(candidate.canonicalize()?);
        }

        // Try with extensions
        for ext in &self.extensions {
            let mut candidate_with_ext = candidate.clone();
            candidate_with_ext.set_extension(&ext[1..]); // Remove the dot

            if candidate_with_ext.exists() {
                return Ok(candidate_with_ext.canonicalize()?);
            }
        }

        // Try as directory with index file
        if candidate.is_dir() {
            for ext in &self.extensions {
                let index_file = candidate.join(format!("index{}", ext));
                if index_file.exists() {
                    return Ok(index_file.canonicalize()?);
                }
            }
        }

        Err(ResolverError::ModuleNotFound {
            request: request.to_string(),
            context: context.display().to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_resolve_relative() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        // Create test files
        fs::write(base_path.join("main.js"), "").unwrap();
        fs::write(base_path.join("helper.js"), "").unwrap();

        let resolver = Resolver::new();
        let main_path = base_path.join("main.js");

        let result = resolver.resolve("./helper.js", &main_path).await.unwrap();
        assert!(result.ends_with("helper.js"));
    }

    #[tokio::test]
    async fn test_resolve_with_extension() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        fs::write(base_path.join("main.js"), "").unwrap();
        fs::write(base_path.join("helper.ts"), "").unwrap();

        let resolver = Resolver::new();
        let main_path = base_path.join("main.js");

        let result = resolver.resolve("./helper", &main_path).await.unwrap();
        assert!(result.ends_with("helper.ts"));
    }
}
