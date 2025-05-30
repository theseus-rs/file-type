use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_372437491: FileType = FileType {
    file_format: &FileFormat {
        id: 372_437_491,
        source_type: SourceType::Iana,
        name: "yang-patch+xml",
        extensions: &[],
        media_types: &["application/yang-patch+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
