use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3280513078: FileFormat = FileFormat {
    id: 3_280_513_078,
    source_type: SourceType::Httpd,
    name: "turtle",
    extensions: &["ttl"],
    media_types: &["text/turtle"],
    signatures: &[],
    related_formats: &[],
};
