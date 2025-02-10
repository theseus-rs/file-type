use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_672123413: FileType = FileType {
    file_format: &FileFormat {
        id: 672_123_413,
        source_type: SourceType::Iana,
        name: "digest",
        extensions: &[],
        media_types: &["multipart/digest"],
        signatures: &[],
        related_formats: &[],
    },
};
