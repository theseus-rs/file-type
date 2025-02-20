use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1125684053: FileType = FileType {
    file_format: &FileFormat {
        id: 1_125_684_053,
        source_type: SourceType::Iana,
        name: "resource-lists-diff+xml",
        extensions: &[],
        media_types: &["application/resource-lists-diff+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
