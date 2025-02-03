use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1319735287: FileFormat = FileFormat {
    id: 1_319_735_287,
    source_type: SourceType::Httpd,
    name: "sparql results xml",
    extensions: &["srx"],
    media_types: &["application/sparql-results+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
