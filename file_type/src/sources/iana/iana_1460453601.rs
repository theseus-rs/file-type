use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1460453601: FileFormat = FileFormat {
    id: 1_460_453_601,
    source_type: SourceType::Iana,
    name: "mbms-envelope+xml",
    extensions: &[],
    media_types: &["application/mbms-envelope+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
