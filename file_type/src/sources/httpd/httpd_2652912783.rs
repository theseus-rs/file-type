use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2652912783: FileFormat = FileFormat {
    id: 2_652_912_783,
    source_type: SourceType::Httpd,
    name: "jsonml json",
    extensions: &["jsonml"],
    media_types: &["application/jsonml+json"],
    signatures: &[],
    related_formats: &[],
};
