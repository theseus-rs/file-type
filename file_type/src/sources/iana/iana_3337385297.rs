use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3337385297: FileFormat = FileFormat {
    id: 3_337_385_297,
    source_type: SourceType::Iana,
    name: "timestamped-data",
    extensions: &[],
    media_types: &["application/timestamped-data"],
    signatures: &[],
    related_formats: &[],
};
