use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2846680230: FileFormat = FileFormat {
    id: 2_846_680_230,
    source_type: SourceType::Httpd,
    name: "mets xml",
    extensions: &["mets"],
    media_types: &["application/mets+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
