use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_15: FileType = FileType {
    file_format: &FileFormat {
        id: 15,
        source_type: SourceType::Linguist,
        name: "Ant Build System",
        extensions: &[],
        media_types: &["application/xml"],
        signatures: &[],
        related_formats: &[],
    },
};
