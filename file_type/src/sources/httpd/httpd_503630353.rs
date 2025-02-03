use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_503630353: FileFormat = FileFormat {
    id: 503_630_353,
    source_type: SourceType::Httpd,
    name: "mp21",
    extensions: &["m21", "mp21"],
    media_types: &["application/mp21"],
    internal_signatures: &[],
    related_formats: &[],
};
