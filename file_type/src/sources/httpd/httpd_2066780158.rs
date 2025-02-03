use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2066780158: FileFormat = FileFormat {
    id: 2_066_780_158,
    source_type: SourceType::Httpd,
    name: "cml",
    extensions: &["cml"],
    media_types: &["chemical/x-cml"],
    internal_signatures: &[],
    related_formats: &[],
};
