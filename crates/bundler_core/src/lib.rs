//! Rust Bundler - A Webpack-like bundler written in Rust
//!
//! This crate provides the core functionality for bundling JavaScript/TypeScript
//! applications, similar to Webpack but implemented in Rust for better performance.

pub mod bundler;
pub mod chunk;
pub mod compiler;
pub mod config;
pub mod dependency;
pub mod error;
pub mod loader;
pub mod module;
pub mod plugin;
pub mod types;

pub use bundler::Bundler;
pub use config::Config;
pub use error::{BundlerError, Result};
pub use module::Module;

pub use bundler::Bundler;
pub use config::{Config, define_config as defineConfig};
pub use error::{BundlerError, Result};
pub use module::Module;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_bundling() {
        let temp_dir = TempDir::new().unwrap();
        let entry_path = temp_dir.path().join("index.js");

        fs::write(
            &entry_path,
            r#"
            import { helper } from './helper.js';
            console.log('Hello from bundler!');
            helper();
        "#,
        )
        .unwrap();

        fs::write(
            temp_dir.path().join("helper.js"),
            r#"
            export function helper() {
                console.log('Helper function called');
            }
        "#,
        )
        .unwrap();

        let config = Config::default()
            .with_entry(entry_path.to_str().unwrap())
            .with_output_path(temp_dir.path().join("dist").to_str().unwrap());

        let bundler = Bundler::new(config).await.unwrap();
        let result = bundler.run().await;

        assert!(result.is_ok());
    }
}

#[cfg(feature = "generate-types")]
pub fn generate_typescript_definitions() -> Result<()> {
    types::generate_types().map_err(|e| BundlerError::ConfigError(e.to_string()))?;
    Ok(())
}
