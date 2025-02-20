use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_113: FileType = FileType {
    file_format: &FileFormat {
        id: 113,
        source_type: SourceType::Linguist,
        name: "Formatted",
        extensions: &["eam.fs", "for"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
