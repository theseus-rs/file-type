use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2057474973: FileFormat = FileFormat {
    id: 2_057_474_973,
    source_type: SourceType::Httpd,
    name: "g3fax",
    extensions: &["g3"],
    media_types: &["image/g3fax"],
    internal_signatures: &[],
    related_formats: &[],
};
