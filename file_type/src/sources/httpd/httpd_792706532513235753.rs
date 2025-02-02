use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_792706532513235753: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rdf xml",
    extensions: &["rdf"],
    media_types: &["application/rdf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
