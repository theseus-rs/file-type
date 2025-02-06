use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3337385297: FileFormat = FileFormat {
    id: 3_337_385_297,
    source_type: SourceType::Httpd,
    name: "timestamped data",
    extensions: &["tsd"],
    media_types: &["application/timestamped-data"],
    signatures: &[],
    related_formats: &[],
};
