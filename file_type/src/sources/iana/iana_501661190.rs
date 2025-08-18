use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_501661190: FileType = FileType {
    file_format: &FileFormat {
        id: 501_661_190,
        source_type: SourceType::Iana,
        name: "vnd.verifier-attestation+jwt",
        extensions: &[],
        media_types: &["application/vnd.verifier-attestation+jwt"],
        signatures: &[],
        related_formats: &[],
    },
};
