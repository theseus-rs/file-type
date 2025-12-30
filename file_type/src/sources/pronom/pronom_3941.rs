use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_3941: FileType = FileType {
    file_format: &FileFormat {
        id: 3_941,
        source_type: SourceType::Pronom,
        name: "Rust Source File",
        extensions: &["rs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
