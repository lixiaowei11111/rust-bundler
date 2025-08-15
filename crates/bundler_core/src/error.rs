use thiserror::Error;

#[derive(Error, Debug)]
pub enum BundlerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Module not found: {path}")]
    ModuleNotFound { path: String },

    #[error("Circular dependency detected: {modules:?}")]
    CircularDependency { modules: Vec<String> },

    #[error("Parse error in {file}: {message}")]
    ParseError { file: String, message: String },

    #[error("Loader error: {0}")]
    LoaderError(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Resolver error: {0}")]
    ResolverError(#[from] bundler_resolver::ResolverError),
}

pub type Result<T> = std::result::Result<T, BundlerError>;
