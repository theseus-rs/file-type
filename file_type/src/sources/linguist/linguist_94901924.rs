use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_94901924: FileType = FileType {
    file_format: &FileFormat {
        id: 94_901_924,
        source_type: SourceType::Linguist,
        name: "TSX",
        extensions: &["tsx"],
        media_types: &["text/jsx"],
        signatures: &[],
        related_formats: &[],
    },
};
