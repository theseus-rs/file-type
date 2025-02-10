use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_309576543: FileType = FileType {
    file_format: &FileFormat {
        id: 309_576_543,
        source_type: SourceType::Iana,
        name: "jxsi",
        extensions: &[],
        media_types: &["image/jxsi"],
        signatures: &[],
        related_formats: &[],
    },
};
