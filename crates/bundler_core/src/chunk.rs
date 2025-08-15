use crate::{Module, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: String,
    pub name: Option<String>,
    pub modules: Vec<String>, // module IDs
    pub entry_points: Vec<String>,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChunkType {
    Entry,
    Normal,
    Async,
}

impl Chunk {
    pub fn new(id: String, chunk_type: ChunkType) -> Self {
        Self {
            id,
            name: None,
            modules: Vec::new(),
            entry_points: Vec::new(),
            chunk_type,
        }
    }

    pub fn add_module(&mut self, module_id: String) {
        if !self.modules.contains(&module_id) {
            self.modules.push(module_id);
        }
    }

    pub fn add_entry_point(&mut self, entry_id: String) {
        if !self.entry_points.contains(&entry_id) {
            self.entry_points.push(entry_id);
        }
    }
}

pub struct ChunkGenerator {
    chunks: Vec<Chunk>,
}

impl ChunkGenerator {
    pub fn new() -> Self {
        Self { chunks: Vec::new() }
    }

    pub fn generate_chunks(&mut self, modules: &[Module]) -> Result<Vec<Chunk>> {
        // For MVP, create a single chunk with all modules
        let mut main_chunk = Chunk::new("main".to_string(), ChunkType::Entry);

        for module in modules {
            main_chunk.add_module(module.id.clone());
            if module.is_entry() {
                main_chunk.add_entry_point(module.id.clone());
            }
        }

        main_chunk.name = Some("main".to_string());
        self.chunks.push(main_chunk.clone());

        Ok(vec![main_chunk])
    }

    pub fn get_chunks(&self) -> &[Chunk] {
        &self.chunks
    }
}
