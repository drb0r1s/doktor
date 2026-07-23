use std::fs;
use std::io;
use std::path::Path;

use crate::frontend::resolved_ast::{ResolvedDoktorNode};

pub struct DoktorbWriter;

impl DoktorbWriter {
    pub fn write_doktorb(resolved_doktor_node: &ResolvedDoktorNode, path_str: &str) -> io::Result<()> {
        let bytes: Vec<u8> = bincode::serialize(&resolved_doktor_node).expect("Failed to serialize ResolvedDoktorNode.");
        
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path_str);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, bytes)?;

        Ok(())
    }
}