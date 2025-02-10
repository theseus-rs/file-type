use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3680692499: FileType = FileType {
    file_format: &FileFormat {
        id: 3_680_692_499,
        source_type: SourceType::Iana,
        name: "vnd.shopkick+json",
        extensions: &[],
        media_types: &["application/vnd.shopkick+json"],
        signatures: &[],
        related_formats: &[],
    },
};
