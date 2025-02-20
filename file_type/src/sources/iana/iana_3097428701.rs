use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3097428701: FileType = FileType {
    file_format: &FileFormat {
        id: 3_097_428_701,
        source_type: SourceType::Iana,
        name: "concise-problem-details+cbor",
        extensions: &[],
        media_types: &["application/concise-problem-details+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
