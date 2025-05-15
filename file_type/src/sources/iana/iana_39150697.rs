use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_39150697: FileType = FileType {
    file_format: &FileFormat {
        id: 39_150_697,
        source_type: SourceType::Iana,
        name: "trust-chain+json",
        extensions: &[],
        media_types: &["application/trust-chain+json"],
        signatures: &[],
        related_formats: &[],
    },
};
