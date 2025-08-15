use crate::{Module, Result};
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::Path;

#[async_trait]
pub trait Loader: Send + Sync {
    async fn load(&self, content: &str, path: &Path) -> Result<String>;
    fn name(&self) -> &str;
}

pub struct LoaderRegistry {
    loaders: HashMap<String, Box<dyn Loader>>,
}

impl LoaderRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            loaders: HashMap::new(),
        };

        // Register default loaders
        registry.register("javascript".to_string(), Box::new(JavaScriptLoader));
        registry.register("typescript".to_string(), Box::new(TypeScriptLoader));
        registry.register("json".to_string(), Box::new(JsonLoader));

        registry
    }

    pub fn register(&mut self, name: String, loader: Box<dyn Loader>) {
        self.loaders.insert(name, loader);
    }

    pub async fn load(&self, loader_name: &str, content: &str, path: &Path) -> Result<String> {
        match self.loaders.get(loader_name) {
            Some(loader) => loader.load(content, path).await,
            None => Err(crate::BundlerError::LoaderError(format!(
                "Loader '{}' not found",
                loader_name
            ))),
        }
    }
}

pub struct JavaScriptLoader;

#[async_trait]
impl Loader for JavaScriptLoader {
    async fn load(&self, content: &str, _path: &Path) -> Result<String> {
        // For MVP, just return the content as-is
        // In a real implementation, you'd parse and transform the JS
        Ok(content.to_string())
    }

    fn name(&self) -> &str {
        "javascript"
    }
}

pub struct TypeScriptLoader;

#[async_trait]
impl Loader for TypeScriptLoader {
    async fn load(&self, content: &str, _path: &Path) -> Result<String> {
        // For MVP, strip type annotations (very basic)
        // In a real implementation, you'd use a proper TS compiler
        let js_content = content
            .lines()
            .map(|line| {
                // Remove type annotations (very basic regex)
                let line = regex::Regex::new(r": \w+").unwrap().replace_all(line, "");
                line.to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(js_content)
    }

    fn name(&self) -> &str {
        "typescript"
    }
}

pub struct JsonLoader;

#[async_trait]
impl Loader for JsonLoader {
    async fn load(&self, content: &str, _path: &Path) -> Result<String> {
        // Convert JSON to ES module
        Ok(format!("export default {};", content))
    }

    fn name(&self) -> &str {
        "json"
    }
}
