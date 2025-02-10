use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1457: FileType = FileType {
    file_format: &FileFormat {
        id: 1_457,
        source_type: SourceType::Pronom,
        name: "Cypher Query Language",
        extensions: &["cql"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
