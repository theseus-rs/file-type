use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_166: FileType = FileType {
    file_format: &FileFormat {
        id: 166,
        source_type: SourceType::Linguist,
        name: "Inform 7",
        extensions: &["i7x", "ni"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
