use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_39150697: FileType = FileType {
    file_format: &FileFormat {
        id: 39_150_697,
        source_type: SourceType::Iana,
        name: "application/trust-chain+json",
        extensions: &[],
        media_types: &["application/trust-chain+json"],
        signatures: &[],
        related_formats: &[],
    },
};
