use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1565024009: FileFormat = FileFormat {
    id: 1_565_024_009,
    source_type: SourceType::Iana,
    name: "rtploopback",
    extensions: &[],
    media_types: &["application/rtploopback"],
    internal_signatures: &[],
    related_formats: &[],
};
