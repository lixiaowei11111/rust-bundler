use crate::{Result, compiler::Compiler, config::Config};
use std::path::Path;

pub struct Bundler {
    config: Config,
    compiler: Compiler,
}

impl Bundler {
    pub async fn new(config: Config) -> Result<Self> {
        let compiler = Compiler::new(config.clone());

        Ok(Self { config, compiler })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Starting bundler...");

        // Ensure output directory exists
        let output_path = Path::new(&self.config.output.path);
        if !output_path.exists() {
            tokio::fs::create_dir_all(output_path).await?;
        }

        // Run compilation
        let result = self.compiler.compile().await?;

        // Write assets to disk
        self.write_assets(&result.assets).await?;

        tracing::info!("Bundling completed successfully");
        Ok(())
    }

    async fn write_assets(&self, assets: &std::collections::HashMap<String, String>) -> Result<()> {
        for (filename, content) in assets {
            let file_path = Path::new(&self.config.output.path).join(filename);
            tokio::fs::write(file_path, content).await?;
            tracing::info!("Generated: {}", filename);
        }
        Ok(())
    }
}
