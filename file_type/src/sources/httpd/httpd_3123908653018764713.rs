use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3123908653018764713: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sparql results xml",
    extensions: &["srx"],
    media_types: &["application/sparql-results+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
