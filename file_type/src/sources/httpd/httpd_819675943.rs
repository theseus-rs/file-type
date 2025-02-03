use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_819675943: FileFormat = FileFormat {
    id: 819_675_943,
    source_type: SourceType::Httpd,
    name: "research info systems",
    extensions: &["ris"],
    media_types: &["application/x-research-info-systems"],
    internal_signatures: &[],
    related_formats: &[],
};
