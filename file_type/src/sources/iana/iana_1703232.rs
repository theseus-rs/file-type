use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1703232: FileType = FileType {
    file_format: &FileFormat {
        id: 1_703_232,
        source_type: SourceType::Iana,
        name: "vnd.geo+json (OBSOLETED by [RFC7946] in favor of application/geo+json)",
        extensions: &[],
        media_types: &["application/vnd.geo+json"],
        signatures: &[],
        related_formats: &[],
    },
};
