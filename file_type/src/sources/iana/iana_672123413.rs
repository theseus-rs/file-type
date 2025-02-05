use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_672123413: FileFormat = FileFormat {
    id: 672_123_413,
    source_type: SourceType::Iana,
    name: "digest",
    extensions: &[],
    media_types: &["multipart/digest"],
    signatures: &[],
    related_formats: &[],
};
