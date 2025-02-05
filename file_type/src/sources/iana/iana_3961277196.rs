use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3961277196: FileFormat = FileFormat {
    id: 3_961_277_196,
    source_type: SourceType::Iana,
    name: "timestamp-query",
    extensions: &[],
    media_types: &["application/timestamp-query"],
    signatures: &[],
    related_formats: &[],
};
