use crate::{Module, Result};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn apply(&self, context: &mut PluginContext) -> Result<()>;
    fn name(&self) -> &str;
}

pub struct PluginContext {
    pub modules: Vec<Module>,
    pub output_path: String,
    pub data: HashMap<String, serde_json::Value>,
}

impl PluginContext {
    pub fn new(output_path: String) -> Self {
        Self {
            modules: Vec::new(),
            output_path,
            data: HashMap::new(),
        }
    }
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub async fn apply_all(&self, context: &mut PluginContext) -> Result<()> {
        for plugin in &self.plugins {
            plugin.apply(context).await?;
        }
        Ok(())
    }
}

// Example plugin for generating HTML
pub struct HtmlPlugin {
    template: String,
}

impl HtmlPlugin {
    pub fn new(template: String) -> Self {
        Self { template }
    }
}

#[async_trait]
impl Plugin for HtmlPlugin {
    async fn apply(&self, context: &mut PluginContext) -> Result<()> {
        let html_content = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Bundled App</title>
</head>
<body>
    <div id="root"></div>
    <script src="bundle.js"></script>
</body>
</html>"#
        );

        let html_path = format!("{}/index.html", context.output_path);
        tokio::fs::write(html_path, html_content).await?;

        Ok(())
    }

    fn name(&self) -> &str {
        "html-plugin"
    }
}
