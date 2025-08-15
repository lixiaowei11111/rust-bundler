use crate::{Result, dependency::Dependency};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub path: PathBuf,
    pub content: String,
    pub dependencies: Vec<Dependency>,
    pub source_map: Option<String>,
    pub module_type: ModuleType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleType {
    JavaScript,
    TypeScript,
    Json,
    Css,
    Asset,
}

impl Module {
    pub fn new(id: String, path: PathBuf, content: String) -> Self {
        let module_type = Self::determine_type(&path);

        Self {
            id,
            path,
            content,
            dependencies: Vec::new(),
            source_map: None,
            module_type,
        }
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        self.dependencies.push(dependency);
    }

    pub fn get_dependency_paths(&self) -> Vec<String> {
        self.dependencies
            .iter()
            .map(|dep| dep.request.clone())
            .collect()
    }

    fn determine_type(path: &PathBuf) -> ModuleType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("js") => ModuleType::JavaScript,
            Some("ts") => ModuleType::TypeScript,
            Some("json") => ModuleType::Json,
            Some("css") => ModuleType::Css,
            _ => ModuleType::Asset,
        }
    }

    pub fn is_entry(&self) -> bool {
        self.id.starts_with("entry:")
    }
}

#[derive(Debug)]
pub struct ModuleGraph {
    modules: Vec<Module>,
    dependencies: Vec<(String, String)>, // (from_id, to_id)
}

impl ModuleGraph {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            dependencies: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: Module) {
        self.modules.push(module);
    }

    pub fn add_dependency(&mut self, from_id: String, to_id: String) {
        self.dependencies.push((from_id, to_id));
    }

    pub fn get_modules(&self) -> &[Module] {
        &self.modules
    }

    pub fn get_entry_modules(&self) -> Vec<&Module> {
        self.modules.iter().filter(|m| m.is_entry()).collect()
    }

    pub fn detect_circular_dependencies(&self) -> Result<()> {
        // Simple cycle detection using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for module in &self.modules {
            if !visited.contains(&module.id) {
                if self.has_cycle(&module.id, &mut visited, &mut rec_stack)? {
                    return Err(crate::BundlerError::CircularDependency {
                        modules: rec_stack.into_iter().collect(),
                    });
                }
            }
        }

        Ok(())
    }

    fn has_cycle(
        &self,
        module_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> Result<bool> {
        visited.insert(module_id.to_string());
        rec_stack.insert(module_id.to_string());

        for (from, to) in &self.dependencies {
            if from == module_id {
                if !visited.contains(to) {
                    if self.has_cycle(to, visited, rec_stack)? {
                        return Ok(true);
                    }
                } else if rec_stack.contains(to) {
                    return Ok(true);
                }
            }
        }

        rec_stack.remove(module_id);
        Ok(false)
    }
}
