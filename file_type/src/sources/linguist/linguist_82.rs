use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_82: FileType = FileType {
    file_format: &FileFormat {
        id: 82,
        source_type: SourceType::Linguist,
        name: "DIGITAL Command Language",
        extensions: &["com"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
