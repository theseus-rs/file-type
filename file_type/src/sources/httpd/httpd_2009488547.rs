use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2009488547: FileFormat = FileFormat {
    id: 2_009_488_547,
    source_type: SourceType::Httpd,
    name: "sparql query",
    extensions: &["rq"],
    media_types: &["application/sparql-query"],
    signatures: &[],
    related_formats: &[],
};
