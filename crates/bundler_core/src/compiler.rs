use crate::{
    Result,
    chunk::{Chunk, ChunkGenerator},
    config::Config,
    loader::LoaderRegistry,
    module::{Module, ModuleGraph},
    plugin::{PluginContext, PluginManager},
};
use bundler_resolver::Resolver;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Compiler {
    config: Config,
    loader_registry: LoaderRegistry,
    plugin_manager: PluginManager,
    resolver: Resolver,
}

impl Compiler {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            loader_registry: LoaderRegistry::new(),
            plugin_manager: PluginManager::new(),
            resolver: Resolver::new(),
        }
    }

    pub async fn compile(&mut self) -> Result<CompilationResult> {
        tracing::info!("Starting compilation...");

        // 1. Build module graph
        let mut module_graph = self.build_module_graph().await?;

        // 2. Check for circular dependencies
        module_graph.detect_circular_dependencies()?;

        // 3. Generate chunks
        let mut chunk_generator = ChunkGenerator::new();
        let chunks = chunk_generator.generate_chunks(module_graph.get_modules())?;

        // 4. Generate output
        let assets = self
            .generate_assets(&chunks, module_graph.get_modules())
            .await?;

        // 5. Apply plugins
        let mut plugin_context = PluginContext::new(self.config.output.path.clone());
        plugin_context.modules = module_graph.get_modules().to_vec();
        self.plugin_manager.apply_all(&mut plugin_context).await?;

        tracing::info!("Compilation completed successfully");

        Ok(CompilationResult {
            chunks,
            assets,
            modules: module_graph.get_modules().to_vec(),
        })
    }

    async fn build_module_graph(&self) -> Result<ModuleGraph> {
        let mut module_graph = ModuleGraph::new();
        let mut processed = std::collections::HashSet::new();

        // Start with entry module
        let entry_path = Path::new(&self.config.entry);
        self.process_module(entry_path, &mut module_graph, &mut processed, true)
            .await?;

        Ok(module_graph)
    }

    async fn process_module(
        &self,
        path: &Path,
        module_graph: &mut ModuleGraph,
        processed: &mut std::collections::HashSet<PathBuf>,
        is_entry: bool,
    ) -> Result<()> {
        let absolute_path = path.canonicalize()?;

        if processed.contains(&absolute_path) {
            return Ok(());
        }

        processed.insert(absolute_path.clone());

        // Read module content
        let content = tokio::fs::read_to_string(&absolute_path).await?;

        // Create module
        let module_id = if is_entry {
            format!("entry:{}", absolute_path.display())
        } else {
            absolute_path.display().to_string()
        };

        let mut module = Module::new(module_id.clone(), absolute_path.clone(), content.clone());

        // Parse dependencies
        let dependencies = self.parse_dependencies(&content, &absolute_path).await?;

        for dep in dependencies {
            module.add_dependency(dep.clone());

            // Resolve dependency path
            if let Ok(resolved_path) = self.resolver.resolve(&dep.request, &absolute_path).await {
                module_graph.add_dependency(module_id.clone(), resolved_path.display().to_string());

                // Recursively process dependency
                self.process_module(&resolved_path, module_graph, processed, false)
                    .await?;
            }
        }

        module_graph.add_module(module);
        Ok(())
    }

    async fn parse_dependencies(
        &self,
        content: &str,
        _path: &Path,
    ) -> Result<Vec<crate::dependency::Dependency>> {
        use crate::dependency::{Dependency, DependencyType};
        use regex::Regex;

        let mut dependencies = Vec::new();

        // Parse ES6 imports
        let import_regex = Regex::new(r#"import\s+.*?\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        for cap in import_regex.captures_iter(content) {
            if let Some(module_path) = cap.get(1) {
                dependencies.push(Dependency::new(
                    module_path.as_str().to_string(),
                    DependencyType::Import,
                ));
            }
        }

        // Parse CommonJS requires
        let require_regex = Regex::new(r#"require\(['"]([^'"]+)['"]\)"#).unwrap();
        for cap in require_regex.captures_iter(content) {
            if let Some(module_path) = cap.get(1) {
                dependencies.push(Dependency::new(
                    module_path.as_str().to_string(),
                    DependencyType::Require,
                ));
            }
        }

        Ok(dependencies)
    }

    async fn generate_assets(
        &self,
        chunks: &[Chunk],
        modules: &[Module],
    ) -> Result<HashMap<String, String>> {
        let mut assets = HashMap::new();

        for chunk in chunks {
            let asset_content = self.generate_chunk_content(chunk, modules).await?;
            let filename = match &chunk.name {
                Some(name) => format!("{}.js", name),
                None => self.config.output.filename.clone(),
            };
            assets.insert(filename, asset_content);
        }

        Ok(assets)
    }

    async fn generate_chunk_content(&self, chunk: &Chunk, modules: &[Module]) -> Result<String> {
        let mut bundle_content = String::new();

        // Generate module map
        bundle_content.push_str("(function(modules) {\n");
        bundle_content.push_str("  var installedModules = {};\n");
        bundle_content.push_str("  function __webpack_require__(moduleId) {\n");
        bundle_content.push_str("    if(installedModules[moduleId]) {\n");
        bundle_content.push_str("      return installedModules[moduleId].exports;\n");
        bundle_content.push_str("    }\n");
        bundle_content.push_str("    var module = installedModules[moduleId] = {\n");
        bundle_content.push_str("      i: moduleId,\n");
        bundle_content.push_str("      l: false,\n");
        bundle_content.push_str("      exports: {}\n");
        bundle_content.push_str("    };\n");
        bundle_content.push_str("    modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);\n");
        bundle_content.push_str("    module.l = true;\n");
        bundle_content.push_str("    return module.exports;\n");
        bundle_content.push_str("  }\n");

        // Add modules
        bundle_content.push_str("  return __webpack_require__(__webpack_require__.s = ");

        if let Some(entry_point) = chunk.entry_points.first() {
            bundle_content.push_str(&format!("'{}'", entry_point));
        } else {
            bundle_content.push_str("0");
        }

        bundle_content.push_str(");\n");
        bundle_content.push_str("})([");

        for (i, module_id) in chunk.modules.iter().enumerate() {
            if i > 0 {
                bundle_content.push_str(",\n");
            }

            if let Some(module) = modules.iter().find(|m| &m.id == module_id) {
                // Transform the module content
                let transformed_content = self
                    .transform_module_content(&module.content, module)
                    .await?;

                bundle_content.push_str(&format!(
                    "/* {} */\nfunction(module, exports, __webpack_require__) {{\n{}\n}}",
                    module_id, transformed_content
                ));
            }
        }

        bundle_content.push_str("]);");
        Ok(bundle_content)
    }

    async fn transform_module_content(&self, content: &str, module: &Module) -> Result<String> {
        // Transform ES6 imports/exports to CommonJS for the runtime
        let mut transformed = content.to_string();

        // Transform imports
        let import_regex =
            regex::Regex::new(r#"import\s+\{([^}]+)\}\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        transformed = import_regex
            .replace_all(&transformed, |caps: &regex::Captures| {
                let imports = &caps[1];
                let module_path = &caps[2];
                format!(
                    "const {{ {} }} = __webpack_require__('{}');",
                    imports, module_path
                )
            })
            .to_string();

        // Transform default imports
        let default_import_regex =
            regex::Regex::new(r#"import\s+(\w+)\s+from\s+['"]([^'"]+)['"]"#).unwrap();
        transformed = default_import_regex
            .replace_all(&transformed, |caps: &regex::Captures| {
                let var_name = &caps[1];
                let module_path = &caps[2];
                format!(
                    "const {} = __webpack_require__('{}');",
                    var_name, module_path
                )
            })
            .to_string();

        // Transform exports
        let export_regex = regex::Regex::new(r"export\s+function\s+(\w+)").unwrap();
        transformed = export_regex
            .replace_all(&transformed, |caps: &regex::Captures| {
                let func_name = &caps[1];
                format!(
                    "function {}\nexports.{} = {};",
                    func_name, func_name, func_name
                )
            })
            .to_string();

        Ok(transformed)
    }
}

pub struct CompilationResult {
    pub chunks: Vec<Chunk>,
    pub assets: HashMap<String, String>,
    pub modules: Vec<Module>,
}
