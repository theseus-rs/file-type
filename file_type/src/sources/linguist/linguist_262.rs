use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_262: FileType = FileType {
    file_format: &FileFormat {
        id: 262,
        source_type: SourceType::Linguist,
        name: "Opal",
        extensions: &["opal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
