use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1799191154: FileFormat = FileFormat {
    id: 1_799_191_154,
    source_type: SourceType::Httpd,
    name: "oda",
    extensions: &["oda"],
    media_types: &["application/oda"],
    internal_signatures: &[],
    related_formats: &[],
};
