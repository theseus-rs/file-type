use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11948227751138753910: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sparql query",
    extensions: &["rq"],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
