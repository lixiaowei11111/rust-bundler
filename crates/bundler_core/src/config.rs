use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub entry: String,
    pub output: OutputConfig,
    pub module: ModuleConfig,
    pub resolve: ResolveConfig,
    pub plugins: Vec<String>,
    pub mode: Mode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub path: String,
    pub filename: String,
    pub chunk_filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub test: String, // regex pattern
    pub use_loader: String,
    pub exclude: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveConfig {
    pub extensions: Vec<String>,
    pub alias: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mode {
    Development,
    Production,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            entry: "./src/index.js".to_string(),
            output: OutputConfig {
                path: "./dist".to_string(),
                filename: "bundle.js".to_string(),
                chunk_filename: "[name].chunk.js".to_string(),
            },
            module: ModuleConfig {
                rules: vec![
                    Rule {
                        test: r"\.js$".to_string(),
                        use_loader: "javascript".to_string(),
                        exclude: Some(r"node_modules".to_string()),
                    },
                    Rule {
                        test: r"\.ts$".to_string(),
                        use_loader: "typescript".to_string(),
                        exclude: Some(r"node_modules".to_string()),
                    },
                ],
            },
            resolve: ResolveConfig {
                extensions: vec![".js".to_string(), ".ts".to_string(), ".json".to_string()],
                alias: HashMap::new(),
            },
            plugins: vec![],
            mode: Mode::Development,
        }
    }
}

impl Config {
    pub async fn from_file(path: &str) -> Result<Self> {
        let content = tokio::fs::read_to_string(path).await?;

        // Check file extension to determine format
        let config = if path.ends_with(".ts") || path.ends_with(".mts") {
            Self::parse_typescript_config(&content).await?
        } else {
            serde_json::from_str(&content)
                .map_err(|e| crate::BundlerError::ConfigError(e.to_string()))?
        };

        Ok(config)
    }

    async fn parse_typescript_config(content: &str) -> Result<Self> {
        // Simple TypeScript config parser
        // In a real implementation, you'd use a proper TS parser

        // Remove TypeScript syntax and convert to JSON-like format
        let mut js_content = content.to_string();

        // Remove import/export statements
        js_content = regex::Regex::new(r"import.*?;")
            .unwrap()
            .replace_all(&js_content, "")
            .to_string();

        js_content = regex::Regex::new(r"export\s+(default\s+)?")
            .unwrap()
            .replace_all(&js_content, "")
            .to_string();

        // Remove type annotations
        js_content = regex::Regex::new(r":\s*\w+(\[\])?")
            .unwrap()
            .replace_all(&js_content, "")
            .to_string();

        // Handle defineConfig wrapper
        if js_content.contains("defineConfig") {
            js_content = regex::Regex::new(r"defineConfig\s*\(\s*")
                .unwrap()
                .replace(&js_content, "")
                .to_string();

            // Remove the last closing parenthesis
            if let Some(last_paren) = js_content.rfind(')') {
                js_content = js_content[..last_paren].to_string();
            }
        }

        // Convert JavaScript object to JSON
        js_content = Self::js_object_to_json(&js_content)?;

        serde_json::from_str(&js_content).map_err(|e| {
            crate::BundlerError::ConfigError(format!("Failed to parse TS config: {}", e))
        })
    }

    fn js_object_to_json(js_content: &str) -> Result<String> {
        // Simple conversion from JS object syntax to JSON
        let mut json_content = js_content.to_string();

        // Add quotes around unquoted keys
        json_content = regex::Regex::new(r"(\w+):")
            .unwrap()
            .replace_all(&json_content, r#""$1":"#)
            .to_string();

        // Convert single quotes to double quotes
        json_content = json_content.replace("'", "\"");

        // Remove comments
        json_content = regex::Regex::new(r"//.*")
            .unwrap()
            .replace_all(&json_content, "")
            .to_string();

        json_content = regex::Regex::new(r"/\*[\s\S]*?\*/")
            .unwrap()
            .replace_all(&json_content, "")
            .to_string();

        Ok(json_content)
    }

    // ... existing code ...
}

// Add helper function for TypeScript configs
pub fn define_config(config: Config) -> Config {
    config
}
