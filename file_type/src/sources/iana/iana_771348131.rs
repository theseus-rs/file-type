use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_771348131: FileType = FileType {
    file_format: &FileFormat {
        id: 771_348_131,
        source_type: SourceType::Iana,
        name: "jwk+json",
        extensions: &[],
        media_types: &["application/jwk+json"],
        signatures: &[],
        related_formats: &[],
    },
};
