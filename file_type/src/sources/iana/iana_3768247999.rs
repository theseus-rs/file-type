use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3768247999: FileFormat = FileFormat {
    id: 3_768_247_999,
    source_type: SourceType::Iana,
    name: "vnd.bekitzur-stech+json",
    extensions: &[],
    media_types: &["application/vnd.bekitzur-stech+json"],
    internal_signatures: &[],
    related_formats: &[],
};
