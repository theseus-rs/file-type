use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
