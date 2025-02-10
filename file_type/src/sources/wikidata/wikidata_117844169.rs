use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117844169: FileType = FileType {
    file_format: &FileFormat {
        id: 117_844_169,
        source_type: SourceType::Wikidata,
        name: "Kofax Group 4 file",
        extensions: &["kfx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
