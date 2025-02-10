use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_429: FileType = FileType {
    file_format: &FileFormat {
        id: 429,
        source_type: SourceType::Linguist,
        name: "ABNF",
        extensions: &["abnf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
