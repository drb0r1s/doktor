use std::fs::File;
use std::io::{self, Write, BufWriter};
use std::path::Path;

use crate::backend::packer::{PACKET_SIZE, PackedPackets};

const SIGNATURE: &[u8; 7] = b"DOKTORB";

pub struct DoktorbWriter;

impl DoktorbWriter {
    pub fn write_doktorb(packed_packets: &PackedPackets, path: &Path) -> io::Result<()> {
        let file: File = File::create(path)?;
        let mut writer: BufWriter<File> = BufWriter::new(file);

        let draw_structures_count: u32 = (packed_packets.numeric_buffer.len() / PACKET_SIZE) as u32;
        let string_table_length: u32 = packed_packets.string_table.len() as u32;

        // Header: signature, draw structures count, string table length. Little-endian.
        writer.write_all(SIGNATURE)?;
        writer.write_all(&draw_structures_count.to_le_bytes())?;
        writer.write_all(&string_table_length.to_le_bytes())?;

        // Numeric Buffer: raw f32 bytes. Little-endian.
        for value in &packed_packets.numeric_buffer {
            writer.write_all(&value.to_le_bytes())?;
        }

        // String table: raw UTF-8 bytes.
        writer.write_all(&packed_packets.string_table)?;

        writer.flush()?;

        Ok(())
    }
}