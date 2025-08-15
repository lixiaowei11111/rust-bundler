//! TypeScript type definitions generator

use crate::config::*;
use std::fs;
use std::path::Path;

pub fn generate_types() -> Result<(), Box<dyn std::error::Error>> {
    let types_content = r#"// Type definitions for rust-bundler
// Project: https://github.com/your-username/rust-bundler

declare module 'bundler_core' {
  export interface Config {
    entry: string;
    output: OutputConfig;
    module: ModuleConfig;
    resolve: ResolveConfig;
    plugins: string[];
    mode: Mode;
  }

  export interface OutputConfig {
    path: string;
    filename: string;
    chunkFilename: string;
  }

  export interface ModuleConfig {
    rules: Rule[];
  }

  export interface Rule {
    test: string;
    useLoader: string;
    exclude?: string;
  }

  export interface ResolveConfig {
    extensions: string[];
    alias: Record<string, string>;
  }

  export type Mode = 'Development' | 'Production';

  export function defineConfig(config: Config): Config;
}
"#;

    // Write to node_modules/@types/bundler_core/index.d.ts
    let types_dir = Path::new("node_modules/@types/bundler_core");
    fs::create_dir_all(types_dir)?;
    fs::write(types_dir.join("index.d.ts"), types_content)?;

    // Also write to examples directory
    let examples_types_dir = Path::new("examples/@types/bundler_core");
    fs::create_dir_all(examples_types_dir)?;
    fs::write(examples_types_dir.join("index.d.ts"), types_content)?;

    println!("✅ Generated TypeScript definitions");
    Ok(())
}
