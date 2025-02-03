use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3097428701: FileFormat = FileFormat {
    id: 3_097_428_701,
    source_type: SourceType::Iana,
    name: "concise-problem-details+cbor",
    extensions: &[],
    media_types: &["application/concise-problem-details+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
