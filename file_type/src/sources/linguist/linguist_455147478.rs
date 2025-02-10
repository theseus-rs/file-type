use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_455147478: FileType = FileType {
    file_format: &FileFormat {
        id: 455_147_478,
        source_type: SourceType::Linguist,
        name: "Lean 4",
        extensions: &["lean"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
