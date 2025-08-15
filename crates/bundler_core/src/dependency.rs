use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub request: String,
    pub dependency_type: DependencyType,
    pub range: Option<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Import,
    Require,
    DynamicImport,
    ImportMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Dependency {
    pub fn new(request: String, dependency_type: DependencyType) -> Self {
        Self {
            request,
            dependency_type,
            range: None,
        }
    }

    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.range = Some(Range { start, end });
        self
    }
}
