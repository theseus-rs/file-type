use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2814014795: FileType = FileType {
    file_format: &FileFormat {
        id: 2_814_014_795,
        source_type: SourceType::Iana,
        name: "jwk-set+json",
        extensions: &[],
        media_types: &["application/jwk-set+json"],
        signatures: &[],
        related_formats: &[],
    },
};
