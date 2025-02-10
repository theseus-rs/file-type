use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_234: FileType = FileType {
    file_format: &FileFormat {
        id: 234,
        source_type: SourceType::Linguist,
        name: "Modula-2",
        extensions: &["mod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
