use crc32fast::Hasher as Crc32Hasher;
use serde::{Deserialize, Serialize};
use std::{fs::File, hash::Hash, io, path::Path};

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub struct ManifestEntry {
    pub path: std::path::PathBuf,
    pub size: usize,
    pub name: String,
    pub hash: u32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Manifest {
    pub hash: u32,
    pub entries: Vec<ManifestEntry>,
}

impl Manifest {
    pub fn new(entries: Vec<ManifestEntry>) -> Self {
        let mut hasher = Crc32Hasher::new();
        entries.iter().for_each(|e| e.hash(&mut hasher));
        let hash = hasher.finalize();
        Self { hash, entries }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let manifest: Self = serde_json::from_reader(file)?;
        Ok(manifest)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), io::Error> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}
