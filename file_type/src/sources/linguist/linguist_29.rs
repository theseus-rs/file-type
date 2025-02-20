use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_29: FileType = FileType {
    file_format: &FileFormat {
        id: 29,
        source_type: SourceType::Linguist,
        name: "Batchfile",
        extensions: &["bat", "cmd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
