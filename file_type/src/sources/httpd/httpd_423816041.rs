use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_423816041: FileFormat = FileFormat {
    id: 423_816_041,
    source_type: SourceType::Httpd,
    name: "wspolicy xml",
    extensions: &["wspolicy"],
    media_types: &["application/wspolicy+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
