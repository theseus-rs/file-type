use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1774385812: FileType = FileType {
    file_format: &FileFormat {
        id: 1_774_385_812,
        source_type: SourceType::Iana,
        name: "geo+json",
        extensions: &[],
        media_types: &["application/geo+json"],
        signatures: &[],
        related_formats: &[],
    },
};
