use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_933: FileType = FileType {
    file_format: &FileFormat {
        id: 933,
        source_type: SourceType::Pronom,
        name: "Obsidium Project File",
        extensions: &["opf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
