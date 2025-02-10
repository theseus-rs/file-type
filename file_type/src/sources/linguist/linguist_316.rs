use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_316: FileType = FileType {
    file_format: &FileFormat {
        id: 316,
        source_type: SourceType::Linguist,
        name: "Racket",
        extensions: &["rkt", "rktd", "rktl", "scrbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
