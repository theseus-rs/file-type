use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_64: FileType = FileType {
    file_format: &FileFormat {
        id: 64,
        source_type: SourceType::Linguist,
        name: "ColdFusion",
        extensions: &["cfm", "cfml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
